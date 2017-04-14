//! Tool to backup and restore data

extern crate app_dirs;
#[macro_use]
extern crate clap;
extern crate colored;
extern crate humansize;
extern crate libhupa;

#[macro_use]
mod macros;
mod common;
mod io;

mod add;
mod remove;
mod modify;
mod print;
mod backup;
mod restore;
mod clean;

use add::*;
use remove::*;
use modify::*;
use print::*;
use backup::*;
use restore::*;
use clean::*;

use clap::AppSettings;
use humansize::file_size_opts;
use humansize::file_size_opts::FileSizeOpts;
use libhupa::*;

const DEFAULT_FSO: FileSizeOpts = FileSizeOpts {
    space: false,
    ..file_size_opts::DECIMAL
};

fn main() {
    // TODO add ability to modify config
    // TODO add ability to modify hupa
    let matches = clap_app!(hupa =>
        (version: crate_version!())
        (author: "Bastien Badzioch <notkild@gmail.com>")
        (about: "Hupa is a tool to backup and restore data")
        (setting: AppSettings::SubcommandRequiredElseHelp)
        (@arg config: -c --config +global +takes_value "Set config path")
        (@arg metadata: --metadata +global +takes_value "Set metadata path")
        (@subcommand add =>
            (about: "Add a new hupa")
            (@arg count: -n --count +takes_value "Set the number of hupa to add"))
        (@subcommand remove =>
            (about: "Remove one or multiple hupas")
            (aliases: &["rm", "del"])
            (@arg hupa: +takes_value +multiple "Hupa(s) to remove"))
        (@subcommand modify =>
            (about: "Modify parameter of an hupa")
            (@arg hupa: +takes_value +multiple "Hupa(s) to modify"))
        (@subcommand backup =>
            (about: "Backup hupa(s)")
            (@arg all: -a --all conflicts_with[hupa] "Backup all hupas")
            (@arg hupa: +takes_value +multiple "Hupa(s) to backup"))
        (@subcommand restore =>
            (about: "Restore hupa(s)")
            (@arg all: -a --all conflicts_with[hupa] "Restore all hupas")
            (@arg hupa: +takes_value +multiple "Hupa(s) to restore")
            (@arg ignore_root: -i --ignore-root "Ignore hupas that need root access, only for unix"))
        (@subcommand generate =>
            (about: "Generate an archive of all hupas")
            (@arg format: -f --format +takes_value possible_value[tar zip] "File format to use for achive")
            (@arg output: -o --output +takes_value "Output directory/file of the created archive"))
        (@subcommand unpack =>
            (about: "Unpack an hupa archive")
            (@arg archive: +required +takes_value "Path to the archive"))
        (@subcommand print =>
            (about: "Print list of all hupas")
            (@arg size: -s --size "Show files sizes"))
        (@subcommand clean =>
            (about: "Clean hupa(s)")
            (@arg all: -a -all "Clean all hupas")
            (@arg hupa: +takes_value +multiple "Hupa(s) to clean"))).get_matches();

    let config_default = Config::default();
    let config = match matches.value_of("config") {
            Some(s) => Config::read_config_from_path(s),
            None => Config::read_config(),
        }
        .unwrap_or(config_default);
    let mut hupas = match read_metadata_from_config(&config) {
        Ok(h) => h,
        Err(_) => Vec::new(),
    };
    if let Some(p) = matches.value_of("metadata") {
        let mut f = ::std::fs::File::open(p).expect(&format!("Can't open {}", p));
        hupas = read_metadata(&mut f, None).unwrap_or(hupas);
    }


    match matches.subcommand() {
        ("add", Some(sub_m)) => {
            add_subcommand(hupas, &config, sub_m);
        }
        ("remove", Some(sub_m)) => {
            remove_subcommand(hupas, &config, sub_m);
        }
        ("modify", Some(sub_m)) => {
            modify_subcommand(hupas, &config, sub_m);
        }
        ("print", Some(sub_m)) => {
            print_subcommand(hupas, sub_m);
        }
        ("backup", Some(sub_m)) => {
            backup_subcommand(&hupas, sub_m);
        }
        ("restore", Some(sub_m)) => {
            restore_subcommand(hupas, sub_m);
        }
        ("clean", Some(sub_m)) => {
            clean_subcommand(&hupas, sub_m);
        }
        (s, _) => println!("`{}` is not supported yet", s),
    }
}
