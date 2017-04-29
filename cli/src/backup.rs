use clap::ArgMatches;
use common::*;
use libhupa::*;

/// Backup subcommand
pub fn backup_subcommand(hupas: &[Hupa], vars: &VarsHandler, sub_m: &ArgMatches) {
    if sub_m.is_present("all") {
        backup(hupas, vars);
    } else if let Some(hupas_names) = sub_m.values_of("hupa") {
        let hupas_names: Vec<String> = hupas_names.map(|s| s.to_string()).collect();
        backup(&resolve_names(&hupas_names, hupas), vars);
    } else {
        let hupas = select_hupas(hupas, "Select hupas to backup");
        backup(&hupas, vars);
    }
}

/// Backup hupas with interface
pub fn backup(hupas: &[Hupa], vars: &VarsHandler) {
    for hupa in hupas {
        exec_hupa(hupa,
                  |h| h.backup(vars),
                  &PrintOrder::OriginToBackup,
                  "Backing up");
    }
}
