use clap::ArgMatches;
use colored::*;
use hupa::*;
use io::*;
use libhupa::*;

/// Remove subcommand
pub fn remove_subcommand(hupas: Vec<Hupa>, config: &Config, sub_m: &ArgMatches) {
    // TODO show to the user which one is remove
    // TODO add security
    let hupas_to_remove = if let Some(hupas_names) = sub_m.values_of("hupa") {
        let hupas_names: Vec<String> = hupas_names.map(|s| s.to_string()).collect();
        resolve_names(&hupas_names, &hupas)
    } else {
        select_hupas(&hupas, "Select hupas to remove")
    };
    let hupas = hupas
        .into_iter()
        .filter(|h| !hupas_to_remove.contains(h))
        .collect::<Vec<Hupa>>();
    for h in &hupas_to_remove {
        println!("{} is now removed.", h.get_name().yellow().bold());
    }
    save_hupas(config, &hupas);
}
