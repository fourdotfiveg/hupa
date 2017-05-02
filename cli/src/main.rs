//! Tool to backup and restore data

extern crate app_dirs;
#[macro_use]
extern crate clap;
extern crate colored;
extern crate humansize;
#[cfg(unix)]
extern crate libc;
extern crate libhupa;

#[macro_use]
mod macros;
mod common;
mod io;

mod add;
mod remove;
mod modify;
mod config;
mod print;
mod backup;
mod restore;
mod clean;
mod vars;

use add::*;
use remove::*;
use modify::*;
use config::*;
use print::*;
use backup::*;
use restore::*;
use clean::*;
use vars::*;

use clap::AppSettings;
use clap::ArgMatches;
use humansize::file_size_opts;
use humansize::file_size_opts::FileSizeOpts;
use libhupa::*;
use std::env;
use std::fs::File;

const DEFAULT_FSO: FileSizeOpts = FileSizeOpts {
    space: false,
    ..file_size_opts::DECIMAL
};

fn main() {
    // TODO exclude by category
    // TODO sort by category
    let matches = clap_app!(hupa =>
        (version: crate_version!())
        (author: "Bastien Badzioch <notkild@gmail.com>")
        (about: "Hupa is a tool to backup and restore data")
        (setting: AppSettings::SubcommandRequiredElseHelp)
        (@arg config: -c --config +global +takes_value "Set config path")
        (@arg metadata: --metadata +global +takes_value "Set metadata path")
        (@arg user: --("as-user") +global +takes_value "Run hupa as another user, only for unix")
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
        (@subcommand config =>
            (about: "Modify config"))
        (@subcommand backup =>
            (about: "Backup hupa(s)")
            (@arg all: -a --all conflicts_with[hupa] "Backup all hupas")
            (@arg hupa: +takes_value +multiple "Hupa(s) to backup"))
        (@subcommand restore =>
            (about: "Restore hupa(s)")
            (@arg all: -a --all conflicts_with[hupa] "Restore all hupas")
            (@arg hupa: +takes_value +multiple "Hupa(s) to restore")
            (@arg ignore_root: -i --("ignore-root") "Ignore hupas that need root access, only for unix"))
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
            (@arg all: -a --all "Clean all hupas")
            (@arg hupa: +takes_value +multiple "Hupa(s) to clean"))
        (@subcommand vars => 
            (about: "Manipulate vars")
            (setting: AppSettings::SubcommandRequiredElseHelp)
            (@subcommand add => (about: "Add var(s)"))
            (@subcommand remove => (about: "Remove var(s)"))
            (@subcommand modify => (about: "Modify var(s)"))
            (@subcommand list => (about: "List var(s)")))
        ).get_matches();

    if let Some(u) = get_arg_recursive(&matches, "user") {
        #[cfg(target_os = "macos")]
        set_home(u.as_str(), "/Users");
        #[cfg(all(not(target_os = "macos"), unix))]
        set_home(u.as_str(), "/home");
    }

    let config_default = Config::default();
    let config = match get_arg_recursive(&matches, "config") {
            Some(s) => Config::read_config_from_path(s),
            None => Config::read_config(),
        }
        .unwrap_or(config_default);
    let vars = if let Ok(mut s) = File::open(&config.vars_path) {
        VarsHandler::read_from_stream(&mut s).unwrap_or(VarsHandler::new(Vec::new()))
    } else {
        VarsHandler::new(Vec::new())
    };
    let mut hupas = match read_metadata_from_config(&config) {
        Ok(h) => h,
        Err(_) => Vec::new(),
    };
    if let Some(p) = matches.value_of("metadata") {
        let mut f = ::std::fs::File::open(p).expect(&format!("Can't open {}", p));
        hupas = read_metadata(&mut f).unwrap_or(hupas);
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
        ("config", _) => {
            config_subcommand(config);
        }
        ("print", Some(sub_m)) => {
            print_subcommand(hupas, sub_m);
        }
        ("backup", Some(sub_m)) => {
            backup_subcommand(&hupas, &vars, sub_m);
        }
        ("restore", Some(sub_m)) => {
            restore_subcommand(hupas, &vars, sub_m);
        }
        ("clean", Some(sub_m)) => {
            clean_subcommand(&hupas, sub_m);
        }
        ("vars", Some(sub_m)) => {
            vars_subcommand(vars, &config, sub_m);
        }
        (s, _) => println!("`{}` is not supported yet", s),
    }
}

fn get_arg_recursive(matches: &ArgMatches, arg: &str) -> Option<String> {
    if let Some(val) = matches.value_of(arg) {
        return Some(val.to_string());
    } else if let Some(sub) = matches.subcommand_name() {
        let sub_m = matches.subcommand_matches(sub).unwrap();
        return get_arg_recursive(sub_m, arg);
    } else {
        None
    }
}

#[cfg(unix)]
fn set_home(user: &str, home: &str) {
    if unsafe { libc::getuid() } != 0 {
        println!("You are not allowed to switch user!");
        let mut args = std::env::args()
            .map(|s| format!("{} ", s))
            .collect::<String>();
        args.pop();
        println!("Run instead `sudo {}`", args);
        return;
    }
    let user = user.trim();
    if user == "0" || user == "root" {
        return;
    }
    if let Ok(id) = user.parse::<u32>() {
        let name = unsafe { (*libc::getpwuid(id)).pw_name };
        let name = unsafe { ::std::ffi::CString::from_raw(name) };
        let name = name.into_string()
            .expect("Cannot convert CString to String");
        env::set_var("HOME", &format!("/{}/{}", home, name));
    } else {
        env::set_var("HOME", &format!("/{}/{}", home, user));
    }
}

#[cfg(not(unix))]
fn set_home(_user: &str) {}
