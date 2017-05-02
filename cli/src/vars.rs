//! Vars subcommand

use clap::ArgMatches;
use libhupa::*;
use std::fs::File;

/// Vars subcommand
pub fn vars_subcommand(mut vars: VarsHandler, config: &Config, sub_m: &ArgMatches) {
    let mut can_write = true;
    match sub_m.subcommand() {
        ("add", _) => vars_add_subcommand(&mut vars),
        ("remove", _) => {}
        ("modify", _) => {}
        ("list", _) => {

            can_write = false;
        }
        (c, _) => println!("Command {} is not supported", c),
    }
    if can_write {
        let mut f = File::create(&config.vars_path).expect("Can't create vars file");
        vars.write_to_stream(&mut f)
            .expect("Can't write vars to file");
    }
}

/// Vars add subcommand
pub fn vars_add_subcommand(vars: &mut VarsHandler) {
    let name = ::io::read_line("Var name: ");
    let value = ::io::read_line_bool("Value (true/false): ", "");
    vars.add_var((name, value));
}
