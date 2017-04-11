#![cfg(unix)]
// TODO support for windows
extern crate app_dirs;
extern crate daemonize;
extern crate libhupa;

use daemonize::Daemonize;
use libhupa::*;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::time::Duration;

fn main() {
    // TODO make daemon receive command from cli
    // TODO make daemon check update
    let config = Config::read_config().unwrap_or(Config::default());
    let hupas = read_metadata_from_config(&config).unwrap();
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
                write!(file, "Waiting {} secs...\n", config.autobackup_interval);
                ::std::thread::sleep(Duration::from_secs(config.autobackup_interval));
            }
        }
        Err(e) => println!("{}", e),
    }
}
