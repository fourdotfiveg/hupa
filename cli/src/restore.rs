use clap::ArgMatches;
use colored::*;
use common::*;
use io::*;
use libhupa::*;
use std::process::Command;

/// Restore subcommand
pub fn restore_subcommand(hupas: Vec<Hupa>, vars: &VarsHandler, sub_m: &ArgMatches) {
    let hupas = if sub_m.is_present("all") {
        hupas
    } else if let Some(hupas_names) = sub_m.values_of("hupa") {
        let hupas_names: Vec<String> = hupas_names.map(|s| s.to_string()).collect();
        resolve_names(&hupas_names, &hupas)
    } else {
        select_hupas(&hupas, "Select hupas to restore")
    };
    #[cfg(not(unix))] restore(&hupas, vars);
    #[cfg(unix)] restore(&hupas, vars, sub_m.is_present("ignore_root"));
}

/// Restore hupas with interface
#[cfg(unix)]
pub fn restore(hupas: &[Hupa], vars: &VarsHandler, ignore_root: bool) {
    // Needs root check
    for hupa in hupas {
        if hupa.needs_root() && !ignore_root {
            println!("Looks like some hupas needs root to be restored.");
            let result = read_line_bool("Ignore them? [y/n]: ");
            if result {
                break;
            } else {
                let mut args = Vec::new();
                args.push(::std::env::args().next().unwrap_or("hupa".to_string()));
                args.push("restore".to_string());
                args.push("--config".to_string());
                args.push(
                    Config::config_path()
                        .expect("Can't get config path")
                        .display()
                        .to_string(),
                );
                for hupa in hupas {
                    args.push(format!("{}/{}", hupa.get_category_str(), hupa.get_name()));
                }
                let mut command = Command::new("sudo");
                let ref_command = command.args(args);
                ref_command
                    .spawn()
                    .expect("Error while spawning sudo command")
                    .wait()
                    .expect("Error while waiting sudo command");
                return;
            }
        }
    }
    for hupa in hupas {
        if hupa.needs_root() {
            println!(
                "{} ignored because he needs root access",
                hupa.get_name().yellow()
            );
            continue;
        }
        exec_hupa(
            hupa,
            |h| h.restore(vars),
            &PrintOrder::BackupToOrigin,
            "Restoring",
        );
    }
}

/// Restore hupas with interface
#[cfg(not(unix))]
pub fn restore(hupas: &[Hupa], vars: &VarsHandler) {
    for hupa in hupas {
        exec_hupa(
            hupa,
            |h| h.restore(vars),
            &PrintOrder::BackupToOrigin,
            "Restoring",
        );
    }
}
