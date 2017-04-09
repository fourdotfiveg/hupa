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
    // TODO read path from config
    // TODO make daemon receive command from cli
    // TODO make daemon check update
    let metadata_path = app_dirs::app_root(app_dirs::AppDataType::UserData, &APP_INFO)
        .unwrap()
        .join("metadata.json");
    let hupas = read_metadata_from_path(&metadata_path);
    let daemonize = Daemonize::new();
    let path = app_dirs::app_root(app_dirs::AppDataType::UserCache, &APP_INFO)
        .unwrap()
        .join("log");
    let mut file = File::create(&path).unwrap();
    match daemonize.start() {
        Ok(_) => {
            loop {
                for hupa in &hupas {
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
                // TODO read time from config
                write!(file, "Waiting 3600 secs...\n");
                ::std::thread::sleep(Duration::from_secs(3600));
            }
        }
        Err(e) => println!("{}", e),
    }
}

/// Read metadata
pub fn read_metadata_from_path(path: &PathBuf) -> Vec<Hupa> {
    let mut f = match File::open(path) {
        Ok(f) => f,
        Err(_) => return Vec::new(),
    };
    libhupa::read_metadata(&mut f, Some(libhupa::MetadataFormat::Json)).unwrap()
}
