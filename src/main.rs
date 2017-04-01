//! Tool to backup and restore data

extern crate app_dirs;
#[macro_use]
extern crate clap;
extern crate hupa;

use hupa::APP_INFO;

use clap::{App, AppSettings, Arg, SubCommand};
use hupa::Hupa;
use std::fs::File;
use std::path::PathBuf;

fn main() {
    let matches = App::new("hupa")
        .about("Hupa is a tool to backup and restore data")
        .author("notkild <notkild@gmail.com>")
        .version(crate_version!())
        .setting(AppSettings::SubcommandRequired)
        .subcommand(SubCommand::with_name("add")
                        .about("Add a new file/directory to backup")
                        .arg(Arg::with_name("file")
                                 .help("The file(s) to backup")
                                 .takes_value(true)
                                 .required(true))
                        .arg(Arg::with_name("hupa")
                                 .help("Set the hupa's name. Format: category/sub_categories[..]")
                                 .takes_value(true)
                                 .required(true)))
        .subcommand(SubCommand::with_name("remove")
                        .aliases(&["rm", "del"])
                        .about("Remove one or multiple hupa")
                        .arg(Arg::with_name("hupa")
                                 .help("Name of the hupa to remove")
                                 .takes_value(true)
                                 .multiple(true)))
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
        .subcommand(SubCommand::with_name("print").about("Print list of all hupas"))
        .get_matches();

    let metadata_path = metadata_path();
    let mut hupas = read_metadata(&metadata_path);

    match matches.subcommand() {
        ("add", Some(sub_m)) => {
            let file = sub_m.value_of("file").unwrap();
            let hupa = sub_m.value_of("hupa").unwrap();
            let mut splitted = hupa.split("/");
            let name = splitted.next().unwrap();
            let mut categories = Vec::new();
            for category in splitted {
                categories.push(category.to_string());
            }
            let hupa = Hupa::new(name, "", categories, file);
            hupas.push(hupa);
            let mut f = File::create(&metadata_path).unwrap();
            hupa::write_metadata(&mut f, &hupas, hupa::MetadataFormat::Json).unwrap();
        }
        ("print", _) => println!("{:?}", hupas),
        (s, _) => println!("`{}` is not supported yet", s),
    }
}

/// Get metadata path
fn metadata_path() -> PathBuf {
    // TODO write path in config
    app_dirs::app_root(app_dirs::AppDataType::UserData, &APP_INFO).unwrap().join("metadata.json")
}

/// Read metadata
fn read_metadata(path: &PathBuf) -> Vec<Hupa> {
    let mut f = match File::open(path) {
        Ok(f) => f,
        Err(_) => return Vec::new(),
    };
    hupa::read_metadata(&mut f, Some(hupa::MetadataFormat::Json)).unwrap()
}
