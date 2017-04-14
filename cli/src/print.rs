use DEFAULT_FSO;
use clap::ArgMatches;
use colored::*;
use humansize::*;
use libhupa::*;

/// Print subcommand
pub fn print_subcommand(hupas: &Vec<Hupa>, sub_m: &ArgMatches) {
    for hupa in hupas {
        let mut size_b = ColoredString::default();
        let mut size_o = ColoredString::default();
        if sub_m.is_present("size") {
            size_b = format!(" ({})",
                             hupa.get_backup_size()
                                 .unwrap_or(0)
                                 .file_size(DEFAULT_FSO)
                                 .expect("Error when showing size"))
                    .bold();
            size_o = format!(" ({})",
                             hupa.get_origin_size()
                                 .unwrap_or(0)
                                 .file_size(DEFAULT_FSO)
                                 .expect("Error when showing size"))
                    .bold();
        }
        let autobackup = if hupa.is_autobackup_enabled() {
            format!("autobackup: {}", "enabled".green())
        } else {
            format!("autobackup: {}", "disabled".red())
        };
        println!("{}/{}{} {} {}{}:\n{}\ndescription: {}\n",
                 hupa.get_categories_str().bold(),
                 hupa.get_name().yellow().bold(),
                 size_b,
                 "<->".bold(),
                 hupa.get_origin().display().to_string().bold(),
                 size_o,
                 autobackup,
                 hupa.get_desc().dimmed());
        hupa.needs_root();
    }

}
