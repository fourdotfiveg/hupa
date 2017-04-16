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
use std::time::Duration;

fn main() {
    // TODO make daemon check update
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
    if let Some(b) = matches.value_of("interval") {
        if let Ok(i) = b.parse() {
            config.autobackup_interval = i;
        }
    }
    let config = config;
    let mut hupas = match read_metadata_from_config(&config) {
        Ok(h) => h,
        Err(_) => Vec::new(),
    };
    if let Some(p) = matches.value_of("metadata") {
        let mut f = ::std::fs::File::open(p).expect(&format!("Can't open {}", p));
        hupas = read_metadata(&mut f, &None).unwrap_or(hupas);
    }

    let daemonize = Daemonize::new();
    let path = app_dirs::app_root(app_dirs::AppDataType::UserCache, &APP_INFO)
        .unwrap()
        .join("log");
    let mut file = File::create(&path).unwrap();
    match daemonize.start() {
        Ok(_) => {
            loop {
                for hupa in &hupas {
                    if !hupa.is_autobackup_enabled() {
                        continue;
                    }
                    match hupa.backup() {
                        Ok(_) => {
                            let _ = write!(file, "{} is backed up\n", hupa.get_name());
                        }
                        Err(e) => {
                            let _ = write!(file,
                                           "{} has an error during backup: {}",
                                           hupa.get_name(),
                                           e);
                        }
                    }
                }
                let _ = write!(file, "Waiting {} secs...\n", config.autobackup_interval);
                ::std::thread::sleep(Duration::from_secs(config.autobackup_interval));
            }
        }
        Err(e) => println!("{}", e),
    }
}
