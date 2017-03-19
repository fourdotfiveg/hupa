//! Tool to backup and restore data

#[macro_use]
extern crate clap;

use clap::{App, AppSettings, Arg, SubCommand};

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
                                 .multiple(true)))
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
        .get_matches();
}
