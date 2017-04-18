use DEFAULT_FSO;
use clap::ArgMatches;
use colored::*;
use humansize::*;
use libhupa::*;

/// Print subcommand
pub fn print_subcommand(hupas: Vec<Hupa>, sub_m: &ArgMatches) {
    let size_enabled = sub_m.is_present("size");
    let mut categories = hupas.into_categories();
    categories.sort();
    for category in categories {
        let mut sizes = Vec::new();
        let mut total = 0;
        let mut total_str = String::new();
        if size_enabled {
            for hupa in &category {
                let size = hupa.get_backup_size().unwrap_or(0);
                total += size;
                sizes.push(size.file_size(DEFAULT_FSO).unwrap());
            }
            total_str = format!(", total {}", total.file_size(DEFAULT_FSO).unwrap());
        } else {
            for _ in 0..category.len() {
                sizes.push(String::new());
            }
        }
        println!("{}: {} item(s){}",
                 category.get_name().bold(),
                 category.len(),
                 total_str);
        for (i, hupa) in category.iter().enumerate() {
            println!(" -- {}:", hupa.get_name().yellow().bold());
            println!("   -- origin: {}", hupa.get_origin().display());
            if size_enabled {
                println!("   -- backup size: {}", sizes[i]);
            }
            let autobackup = if hupa.is_autobackup_enabled() {
                format!("{}", "enabled".green())
            } else {
                format!("{}", "disabled".red())
            };
            println!("   -- autobackup is {}", autobackup);
            println!("   -- description: {}", hupa.get_desc());
        }
    }
}
