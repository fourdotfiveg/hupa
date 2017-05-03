//! Vars subcommand

use clap::ArgMatches;
use libhupa::*;
use std::fs::File;

/// Vars subcommand
pub fn vars_subcommand(mut vars: VarsHandler, config: &Config, sub_m: &ArgMatches) {
    let mut can_write = true;
    match sub_m.subcommand() {
        ("add", _) => vars_add_subcommand(&mut vars),
        ("remove", _) => vars_remove_subcommand(&mut vars),
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

/// Vars remove subcommand
pub fn vars_remove_subcommand(vars: &mut VarsHandler) {
    for i in 0..vars.len() {
        println!("[{}] {}", i + 1, vars[i].0);
    }
    println!("[{}] Cancel", vars.len() + 1);
    let idxs = ::io::read_line_usize("Var(s) to remove: ", "", vars.len());
    let mut vars_to_remove = Vec::new();
    if idxs.contains(&(vars.len() + 1)) {
        println!("Action cancelled");
        return;
    }
    for i in idxs {
        let &(ref name, _) = &vars[i - 1];
        vars_to_remove.push(name.clone());
    }
    for name in vars_to_remove {
        println!("Var {} removed", name);
        vars.remove_var(name);
    }
}
