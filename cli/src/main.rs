//! Tool to backup and restore data

extern crate app_dirs;
#[macro_use]
extern crate clap;
extern crate colored;
extern crate humansize;
extern crate libhupa as hupa;

use hupa::APP_INFO;

use clap::{App, AppSettings, Arg, SubCommand};
use colored::*;
use humansize::{FileSize, file_size_opts};
use humansize::file_size_opts::FileSizeOpts;
use hupa::*;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

const DEFAULT_FSO: FileSizeOpts = FileSizeOpts {
    space: false,
    ..file_size_opts::DECIMAL
};

fn main() {
    let matches = App::new("hupa")
        .about("Hupa is a tool to backup and restore data")
        .author("notkild <notkild@gmail.com>")
        .version(crate_version!())
        .setting(AppSettings::SubcommandRequired)
        .subcommand(SubCommand::with_name("add").about("Add a new file/directory to backup"))
        .subcommand(SubCommand::with_name("remove")
                        .aliases(&["rm", "del"])
                        .about("Remove one or multiple hupa"))
        .subcommand(SubCommand::with_name("backup")
                        .about("Backup hupa(s)")
                        .arg(Arg::with_name("all")
                                 .help("Backup all hups")
                                 .short("a")
                                 .long("all")
                                 .conflicts_with("hupa"))
                        .arg(Arg::with_name("hupa")
                                 .help("Hupa(s) to backup")
                                 .takes_value(true)
                                 .multiple(true)))
        .subcommand(SubCommand::with_name("restore")
                        .about("Restore hupa(s)")
                        .arg(Arg::with_name("all")
                                 .help("Restore all hupa")
                                 .short("a")
                                 .long("all")
                                 .conflicts_with("hupa"))
                        .arg(Arg::with_name("hupa")
                                 .help("Hupa(s) to restore")
                                 .takes_value(true)
                                 .multiple(true)))
        .subcommand(SubCommand::with_name("generate")
                        .about("Generate an archive of all hupas")
                        .arg(Arg::with_name("format")
                                 .short("f")
                                 .long("format")
                                 .help("File format to use for archive")
                                 .takes_value(true))
                        .arg(Arg::with_name("output")
                                 .short("o")
                                 .long("output")
                                 .help("Output directory/file of the created archive")
                                 .takes_value(true)))
        .subcommand(SubCommand::with_name("unpack")
                        .about("Unpack an hupa archive")
                        .arg(Arg::with_name("archive")
                                 .help("Path to the archive")
                                 .required(true)
                                 .takes_value(true)))
        .subcommand(SubCommand::with_name("print")
                        .about("Print list of all hupas")
                        .arg(Arg::with_name("size")
                                 .help("Show files sizes")
                                 .short("s")
                                 .long("size")))
        .subcommand(SubCommand::with_name("clean")
                        .about("Clean hupa(s)")
                        .arg(Arg::with_name("all")
                                 .help("Clean all hupas")
                                 .short("a")
                                 .long("all"))
                        .arg(Arg::with_name("hupa")
                                 .help("Hupa(s) to clean")
                                 .takes_value(true)
                                 .multiple(true)))
        .get_matches();

    let metadata_path = metadata_path();
    let mut hupas = read_metadata(&metadata_path);

    match matches.subcommand() {
        ("add", _) => {
            let name = read_line("Name: ");
            let desc = read_line("Description: ");
            let categories = read_line("Categories (ex: os/linux): ");
            let origin = read_line("Origin path: ");
            let hupa = Hupa::new(name,
                                 desc,
                                 categories.split('/').map(|s| s.to_string()).collect(),
                                 origin);
            hupas.push(hupa);
            save_hupas(&metadata_path, &hupas);
        }
        ("remove", _) => {
            for (i, hupa) in hupas.iter().enumerate() {
                println!("[{}] {}: {}", i + 1, hupa.get_name(), hupa.get_desc());
            }
            println!("[{}] Cancel", hupas.len() + 1);
            loop {
                let idx = read_line_parse::<usize>(&format!("Hupa to remove [1-{}]: ",
                                                            hupas.len() + 1),
                                                   &format!("You should enter a number between 1 and {}",
                                                            hupas.len() + 1));
                if idx == 0 || idx > hupas.len() + 1 {
                    println!("{}", "This is not in the range".red());
                    continue;
                } else if idx == hupas.len() + 1 {
                    println!("Action cancelled");
                    break;
                } else {
                    hupas.remove(idx - 1);
                    save_hupas(&metadata_path, &hupas);
                    break;
                }
            }
        }
        ("print", Some(sub_m)) => {
            for hupa in &hupas {
                let mut size_b = ColoredString::default();
                let mut size_o = ColoredString::default();
                if sub_m.is_present("size") {
                    size_b = format!(" ({})",
                                     hupa.get_backup_size()
                                         .unwrap_or(0)
                                         .file_size(DEFAULT_FSO)
                                         .unwrap())
                            .bold();
                    size_o = format!(" ({})",
                                     hupa.get_origin_size()
                                         .unwrap_or(0)
                                         .file_size(DEFAULT_FSO)
                                         .unwrap())
                            .bold();
                }
                println!("{}/{}{} {} {}{}: \n{}\n",
                         hupa.get_categories_str().bold(),
                         hupa.get_name().yellow().bold(),
                         size_b,
                         "<->".bold(),
                         hupa.get_origin().display().to_string().bold(),
                         size_o,
                         hupa.get_desc().dimmed());
            }
        }
        ("backup", Some(sub_m)) => {
            if sub_m.is_present("all") {
                backup(&hupas);
            } else if let Some(hupas_names) = sub_m.values_of("hupa") {
                let hupas_names: Vec<String> = hupas_names.map(|s| s.to_string()).collect();
                backup(&resolve_names(&hupas_names, &hupas));
            } else {
                // TODO put prompt for choosing
            }
        }
        ("restore", Some(sub_m)) => {
            if sub_m.is_present("all") {
                restore(&hupas);
            } else if let Some(hupas_names) = sub_m.values_of("hupa") {
                let hupas_names: Vec<String> = hupas_names.map(|s| s.to_string()).collect();
                restore(&resolve_names(&hupas_names, &hupas));
            } else {
                // TODO put prompt for choosing
            }

        }
        ("clean", Some(sub_m)) => {
            if sub_m.is_present("all") {
                clean(&hupas);
            } else if let Some(hupas_names) = sub_m.values_of("hupa") {
                let hupas_names: Vec<String> = hupas_names.map(|s| s.to_string()).collect();
                clean(&resolve_names(&hupas_names, &hupas));
            } else {
            }
        }
        (s, _) => println!("`{}` is not supported yet", s),
    }
}

/// Get metadata path
fn metadata_path() -> PathBuf {
    // TODO write path in config
    app_dirs::app_root(app_dirs::AppDataType::UserData, &APP_INFO)
        .unwrap()
        .join("metadata.json")
}

/// Return list of hupa from hupas_names
fn resolve_names(hupas_names: &[String], hupas: &[Hupa]) -> Vec<Hupa> {
    let mut resolved = Vec::new();
    for hupa_name in hupas_names {
        let mut found = false;
        for hupa in hupas {
            if hupa.get_name() == hupa_name {
                resolved.push(hupa.clone());
                found = true;
                break;
            }
        }
        if !found {
            println!("Can't find hupa for name {}", hupa_name);
        }
    }
    resolved
}

/// Read metadata
fn read_metadata(path: &PathBuf) -> Vec<Hupa> {
    let mut f = match File::open(path) {
        Ok(f) => f,
        Err(_) => return Vec::new(),
    };
    hupa::read_metadata(&mut f, Some(hupa::MetadataFormat::Json)).unwrap()
}

/// Backup hupas with interface
fn backup(hupas: &[Hupa]) {
    let mut stdout = ::std::io::stdout();
    for hupa in hupas {
        write!(stdout,
               "Backing up {} ({})... ",
               hupa.get_name().yellow(),
               hupa.get_origin_size()
                   .unwrap_or(0)
                   .file_size(DEFAULT_FSO)
                   .unwrap())
                .unwrap();
        stdout.flush().unwrap();
        match hupa.backup() {
            Ok(_) => {
                write!(stdout, "{}", "OK!".green()).unwrap();
                stdout.flush().unwrap();
            }
            Err(e) => {
                write!(stdout, "{}", "Error: ".red()).unwrap();
                stdout.write(e.description().as_bytes()).unwrap();
                stdout.flush().unwrap();
            }
        }
        stdout.write(b"\n").unwrap();
        stdout.flush().unwrap();
    }
}

/// Restore hupas with interface
fn restore(hupas: &[Hupa]) {
    let mut stdout = ::std::io::stdout();
    for hupa in hupas {
        write!(stdout,
               "Restoring {} ({})... ",
               hupa.get_name().yellow(),
               hupa.get_backup_size()
                   .unwrap_or(0)
                   .file_size(DEFAULT_FSO)
                   .unwrap())
                .unwrap();
        stdout.flush().unwrap();
        match hupa.restore() {
            Ok(_) => {
                write!(stdout, "{}", "OK!".green()).unwrap();
                stdout.flush().unwrap();
            }
            Err(e) => {
                write!(stdout, "{}", "Error: ".red()).unwrap();
                stdout.write(e.description().as_bytes()).unwrap();
                stdout.flush().unwrap();
            }
        }
        stdout.write(b"\n").unwrap();
        stdout.flush().unwrap();
    }
}

/// Clean hupas with interface
fn clean(hupas: &[Hupa]) {
    let mut stdout = ::std::io::stdout();
    for hupa in hupas {
        write!(stdout,
               "Cleaning {} ({})... ",
               hupa.get_name().yellow(),
               hupa.get_backup_size()
                   .unwrap_or(0)
                   .file_size(DEFAULT_FSO)
                   .unwrap())
                .unwrap();
        stdout.flush().unwrap();
        match hupa.delete_backup() {
            Ok(_) => {
                write!(stdout, "{}", "OK!".green()).unwrap();
                stdout.flush().unwrap();
            }
            Err(e) => {
                write!(stdout, "{}", "Error: ".red()).unwrap();
                stdout.write(e.description().as_bytes()).unwrap();
                stdout.flush().unwrap();
            }
        }
        stdout.write(b"\n").unwrap();
        stdout.flush().unwrap();
    }
}

/// Read line
fn read_line(print: &str) -> String {
    let stdin = ::std::io::stdin();
    let mut stdout = ::std::io::stdout();
    let mut buf = String::new();
    while buf.is_empty() {
        stdout.write(print.as_bytes()).unwrap();
        stdout.flush().unwrap();
        stdin.read_line(&mut buf).unwrap();
        buf = buf.trim().to_string()
    }
    buf
}

/// Read line parse
fn read_line_parse<T: ::std::str::FromStr>(print: &str, err_msg: &str) -> T {
    loop {
        let readed = read_line(print);
        if let Ok(r) = readed.parse::<T>() {
            return r;
        } else {
            println!("{}", err_msg.red())
        }
    }
}

/// Save hupas
fn save_hupas(path: &PathBuf, hupas: &Vec<Hupa>) {
    let mut f = File::create(path).unwrap();
    hupa::write_metadata(&mut f, &hupas, hupa::MetadataFormat::Json).unwrap();
}
