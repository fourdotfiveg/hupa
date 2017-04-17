use clap::ArgMatches;
use colored::*;
use io::*;
use libhupa::*;

/// Add subcommand
pub fn add_subcommand(mut hupas: Vec<Hupa>, config: &Config, sub_m: &ArgMatches) {
    let count = sub_m
        .value_of("count")
        .unwrap_or("1")
        .parse::<usize>()
        .unwrap_or(1);
    let mut categories = Vec::new();
    for hupa in &hupas {
        let mut category = hupa.get_categories()
            .iter()
            .map(|s| format!("{}/", s))
            .collect::<String>();
        category.pop();
        categories.push(category);
    }
    categories.sort();
    categories.dedup();
    'main: for _ in 0..count {
        let name = read_line("Name: ");
        let desc = read_line("Description: ");
        println!("Already used categories:");
        for category in &categories {
            println!("- {}", category);
        }
        let categories = read_line("Categories (ex: os/linux): ");
        let origin = read_line("Origin path: ");
                #[cfg(unix)]
        let origin = origin.replace('~', env!("HOME"));
        let autobackup = read_line_bool("Enable autobackup (y/n)? ", "The answer is yes or no");
        let hupa =
            Hupa::new(name.clone(),
                      desc,
                      categories.split('/').map(|s| s.to_string()).collect(),
                      Hupa::get_default_backup_parent().expect("Can't get default backup parent"),
                      origin,
                      autobackup);
        for hupa_stored in &hupas {
            if hupa_stored.get_name() == hupa.get_name() &&
               hupa_stored.get_categories() == hupa.get_categories() &&
               hupa_stored.get_backup_parent() == hupa.get_backup_parent() {
                println!("{}", "This hupa is already set!".red());
                continue 'main;
            }
        }
        println!("{} is now added.", name.yellow());
        hupas.push(hupa);
    }
    save_hupas(config, &hupas);

}
