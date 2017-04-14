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
        if !hupas_to_modify.contains(&hupa) {
            continue;
        }
        println!("Hupa {}:", hupa.get_name().yellow().bold());
        println!("[1] Set name");
        println!("[2] Set desc");
        println!("[3] Set categories");
        println!("[4] Set backup parent");
        println!("[5] Set origin path");
        println!("[6] Set autobackup");
        println!("[7] Cancel");
        let idxs = read_line_usize("Select action [1-7]: ", "", 7);
        if idxs.contains(&7) {
            continue;
        }
        for i in idxs {
            match i {
                1 => {
                    let name = read_line("New name: ");
                    hupa.set_name(name).expect("Cannot rename hupa");
                }
                _ => {}
            }
        }
    }
    save_hupas(config, &hupas);
    // TODO
}
