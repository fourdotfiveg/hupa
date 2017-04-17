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
        println!("[7] Cancel");
        let idxs = read_line_usize("Select action [1-7]: ", "", 7);
        for i in idxs {
            match i {
                1 => {
                    println!("Current name: {}", hupa.get_name());
                    hupa.set_name(read_line("New name: "))
                        .expect("Cannot rename hupa");
                }
                2 => {
                    println!("Current desc: {}", hupa.get_desc());
                    hupa.set_desc(read_line("New description: "));
                }
                3 => {
                    println!("Current category: {}",
                             hupa.get_category()
                                 .iter()
                                 .map(|s| format!("{}/", s))
                                 .collect::<String>());
                    let category = read_line("New category (ex: os/linux): ");
                    hupa.set_category(category.split('/').map(|s| s.to_string()).collect())
                        .expect("Cannot reset category");
                }
                4 => {
                    println!("Current backup parent: {}",
                             hupa.get_backup_parent().display());
                    hupa.set_backup_parent(read_line("New backup parent: "))
                        .expect("Cannot reset backup parent");
                }
                5 => {
                    println!("Current origin path: {}", hupa.get_origin().display());
                    hupa.set_origin_path(read_line("New origin path: "));
                }
                6 => {
                    let print = if hupa.is_autobackup_enabled() {
                        "enabled"
                    } else {
                        "disabled"
                    };
                    println!("Current autobackup state: {}", print);
                    hupa.set_autobackup(read_line_bool("Enable autobackup? [y/n]: ", ""));
                }
                _ => {}
            }
        }
    }
    save_hupas(config, &hupas);
}
