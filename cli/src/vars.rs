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
        ("modify", _) => vars_modify_subcommand(&mut vars),
        ("list", _) => {
            vars_list_subcommand(&vars);
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
    let idxs = select_vars(vars);
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

/// Vars modify subcommand
pub fn vars_modify_subcommand(vars: &mut VarsHandler) {
    let idxs = select_vars(vars);
    if idxs.contains(&(vars.len() + 1)) {
        println!("Action cancelled");
        return;
    }
    for i in idxs {
        let var = &mut vars[i - 1];
        println!("{} = {}", var.0, var.1);
        let new_val = ::io::read_line_bool("New value: ", "");
        var.1 = new_val;
    }
}

/// Vars list subcommand
pub fn vars_list_subcommand(vars: &VarsHandler) {
    for var in vars {
        println!("{} = {}", var.0, var.1);
    }
}

/// List vars and ask for choice
fn select_vars(vars: &VarsHandler) -> Vec<usize> {
    for i in 0..vars.len() {
        println!("[{}] {}", i + 1, vars[i].0);
    }
    println!("[{}] Cancel", vars.len() + 1);
    ::io::read_line_usize("Var(s) to remove: ", "", vars.len())
}
