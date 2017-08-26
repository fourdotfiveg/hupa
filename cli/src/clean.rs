use clap::ArgMatches;
use common::*;
use libhupa::*;

/// Clean subcommand
pub fn clean_subcommand(hupas: &[Hupa], sub_m: &ArgMatches) {
    if sub_m.is_present("all") {
        clean(hupas);
    } else if let Some(hupas_names) = sub_m.values_of("hupa") {
        let hupas_names: Vec<String> = hupas_names.map(|s| s.to_string()).collect();
        clean(&resolve_names(&hupas_names, hupas));
    } else {
        let hupas = select_hupas(hupas, "Select hupas to clean");
        clean(&hupas);
    }
}

/// Clean hupas with interface
pub fn clean(hupas: &[Hupa]) {
    for hupa in hupas {
        exec_hupa(hupa,
                  |h| h.delete_backup(),
                  &PrintOrder::BackupToNull,
                  "Cleaning");
    }
}
