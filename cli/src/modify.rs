use clap::ArgMatches;
use common::*;
use colored::*;
use io::*;
use libhupa::*;

/// Modify subcommand
pub fn modify_subcommand(mut hupas: Vec<Hupa>, config: &Config, sub_m: &ArgMatches) {
    let hupas_to_modify = if let Some(hupas_names) = sub_m.values_of("hupa") {
        let hupas_names: Vec<String> = hupas_names.map(|s| s.to_string()).collect();
        resolve_names(&hupas_names, &hupas)
    } else {
        select_hupas(&hupas, "Select hupas to modify")
    };
    for hupa in &mut hupas {
        if !hupas_to_modify.contains(hupa) {
            continue;
        }
        println!("Hupa {}:", hupa.get_name().yellow().bold());
        println!("[1] Set name");
        println!("[2] Set description");
        println!("[3] Set category");
        println!("[4] Set backup parent");
        println!("[5] Set origin path");
        println!("[6] Set autobackup");
        println!("[7] Set needed vars");
        println!("[8] Cancel");
        let idxs = read_line_usize("Select action [1-8]: ", false, 8);
        for i in idxs {
            match i {
                1 => {
                    println!("Current name: {}", hupa.get_name());
                    hupa.set_name(read_line("New name: ", true)).expect(
                        "Cannot rename hupa",
                    );
                }
                2 => {
                    println!("Current desc: {}", hupa.get_desc());
                    hupa.set_desc(read_line("New description: ", false));
                }
                3 => {
                    println!(
                        "Current category: {}",
                        hupa.get_category()
                            .iter()
                            .map(|s| format!("{}/", s))
                            .collect::<String>()
                    );
                    let category = read_line("New category (ex: os/linux): ", true);
                    hupa.set_category(category.split('/').map(|s| s.to_string()).collect())
                        .expect("Cannot reset category");
                }
                4 => {
                    println!(
                        "Current backup parent: {}",
                        hupa.get_backup_parent().display()
                    );
                    hupa.set_backup_parent(read_line("New backup parent: ", true))
                        .expect("Cannot reset backup parent");
                }
                5 => {
                    println!("Current origin path: {}", hupa.get_origin().display());
                    hupa.set_origin_path(read_line("New origin path: ", true));
                }
                6 => {
                    let print = if hupa.is_autobackup_enabled() {
                        "enabled"
                    } else {
                        "disabled"
                    };
                    println!("Current autobackup state: {}", print);
                    hupa.set_autobackup(read_line_bool("Enable autobackup? [y/n]: "));
                }
                7 => {
                    println!(
                        "Current needed vars: {}",
                        hupa.get_needed_vars()
                            .iter()
                            .map(|s| format!("{} ", s))
                            .collect::<String>()
                    );
                    let needed_vars = read_line("New needed vars: ", false)
                        .split_whitespace()
                        .map(|s| s.to_string())
                        .collect();
                    hupa.set_needed_vars(needed_vars);
                }
                _ => {}
            }
        }
    }
    save_hupas(config, &hupas);
}
