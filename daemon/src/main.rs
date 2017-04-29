#![cfg(unix)]
// TODO support for windows
extern crate app_dirs;
#[macro_use]
extern crate clap;
extern crate daemonize;
extern crate libhupa;

use daemonize::Daemonize;
use libhupa::*;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Duration;
use std::time::SystemTime;

fn main() {
    let matches = clap_app!(hupad =>
            (version: crate_version!())
            (author: "Bastien Badzioch <notkild@gmail.com>")
            (about: "Hupa daemon is used as a server or as a backup daemon")
            (@arg config: -c --config +takes_value "Set config path")
            (@arg metadata: -m --metadata +takes_value "Set metadata path")
            (@arg interval: -i --interval +takes_value "Set backup interval")
        )
            .get_matches();
    let config_default = Config::default();
    let mut config = match matches.value_of("config") {
            Some(s) => Config::read_config_from_path(s),
            None => Config::read_config(),
        }
        .unwrap_or(config_default);
    let mut vars = if let Ok(mut s) = File::open(&config.vars_path) {
        VarsHandler::read_from_stream(&mut s).unwrap_or(VarsHandler::new(Vec::new()))
    } else {
        VarsHandler::new(Vec::new())
    };
    if let Some(b) = matches.value_of("interval") {
        if let Ok(i) = b.parse() {
            config.autobackup_interval = i;
        }
    }
    if let Some(p) = matches.value_of("metadata") {
        config.metadata_path = PathBuf::from(p);
    }
    let config = config;
    let mut hupas = match read_metadata_from_config(&config) {
        Ok(h) => h,
        Err(_) => Vec::new(),
    };

    let start = SystemTime::now();

    let daemonize = Daemonize::new();
    let path = app_dirs::app_root(app_dirs::AppDataType::UserCache, &APP_INFO)
        .unwrap()
        .join("log");
    let mut file = File::create(&path).unwrap();
    match daemonize.start() {
        Ok(_) => {
            let mut last_change_met = get_last_change(&config.metadata_path);
            let mut last_change_vars = get_last_change(&config.vars_path);
            loop {
                let change_met = get_last_change(&config.metadata_path);
                let change_vars = get_last_change(&config.vars_path);

                // Check change metadata
                if last_change_met != change_met {
                    let elapsed = start.elapsed().unwrap().as_secs();
                    let _ = write!(file, "[{}] Found new change in metadata...", elapsed);
                    hupas = match read_metadata_from_config(&config) {
                        Ok(h) => h,
                        Err(_) => hupas,
                    };
                    last_change_met = change_met;
                }

                // Check change vars
                if last_change_vars != change_vars {
                    let elapsed = start.elapsed().unwrap().as_secs();
                    let _ = write!(file, "[{}] Found new change in vars...", elapsed);
                    vars = if let Ok(mut s) = File::open(&config.vars_path) {
                        VarsHandler::read_from_stream(&mut s)
                            .unwrap_or(VarsHandler::new(Vec::new()))
                    } else {
                        VarsHandler::new(Vec::new())
                    };
                    last_change_vars = change_vars;
                }
                for hupa in &hupas {
                    if !hupa.is_autobackup_enabled() {
                        continue;
                    }
                    let elapsed = start.elapsed().unwrap().as_secs();
                    match hupa.backup(&vars) {
                        Ok(_) => {
                            let _ =
                                write!(file, "[{}] {} is backed up\n", elapsed, hupa.get_name());
                        }
                        Err(e) => {
                            let _ = write!(file,
                                           "[{}] {} has an error during backup: {}",
                                           elapsed,
                                           hupa.get_name(),
                                           e);
                        }
                    }
                }
                let elapsed = start.elapsed().unwrap().as_secs();
                let _ = write!(file,
                               "[{}] Waiting {} secs...\n",
                               elapsed,
                               config.autobackup_interval);
                ::std::thread::sleep(Duration::from_secs(config.autobackup_interval));
            }
        }
        Err(e) => write!(file, "Error: {}", e).expect("Can't write to file"),
    }
}

fn get_last_change<P: AsRef<Path>>(path: P) -> SystemTime {
    let metadata = path.as_ref()
        .metadata()
        .expect("Can't get metadata info");
    metadata
        .modified()
        .expect("Can't get last time modified")
}
