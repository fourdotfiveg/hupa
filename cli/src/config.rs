use io::*;
use libhupa::*;
use std::path::Path;

/// Config subcommand
pub fn config_subcommand(mut config: Config) {
    println!("[1] Set metadata path");
    println!("[2] Set metadata format");
    println!("[3] Set autobackup interval");
    println!("[4] Cancel");
    let idxs = read_line_usize("Choose what to change [1-4]: ", "", 4);
    // TODO show current value
    for i in idxs {
        match i {
            1 => {
                let path = read_line("New metadata path: ");
                let path = Path::new(&path);
                config.metadata_path = path.to_path_buf();
            }
            2 => {
                println!("Possible values: ");
                println!("[1] Json");
                let result = read_line_parse::<usize>("Choice [1-1]: ", "");
                if result == 1 {
                    config.metadata_format = MetadataFormat::Json;
                }
            }
            3 => {
                config.autobackup_interval = read_line_parse("Autobackup interval in seconds: ",
                                                             "");
            }
            _ => {}
        }
    }
    // TODO add security
    config.save_config().expect("Can't save config");
}
