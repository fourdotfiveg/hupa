use clap::ArgMatches;
use colored::*;
use io::*;
use libhupa::*;

/// Add subcommand
pub fn add_subcommand(mut hupas: Vec<Hupa>,
                      config: &Config,
                      vars: &VarsHandler,
                      sub_m: &ArgMatches) {
    let count = sub_m
        .value_of("count")
        .unwrap_or("1")
        .parse::<usize>()
        .unwrap_or(1);
    let mut category = Vec::new();
    for hupa in &hupas {
        category.push(hupa.get_category_str());
    }
    category.sort();
    category.dedup();
    'main: for _ in 0..count {
        let name = read_line("Name: ");
        let desc = read_line("Description: ");
        println!("Already used category:");
        for category in &category {
            println!("- {}", category);
        }
        let category = read_line("Categories (ex: os/linux): ");
        let origin = read_line("Origin path: ");
                #[cfg(unix)]
        let origin = origin.replace('~', env!("HOME"));
        let autobackup = read_line_bool("Enable autobackup (y/n)? ", "The answer is yes or no");
        let needed_vars = read_line("Needed vars: ")
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        let hupa =
            Hupa::new(name.clone(),
                      desc,
                      category.split('/').map(|s| s.to_string()).collect(),
                      Hupa::get_default_backup_parent().expect("Can't get default backup parent"),
                      origin,
                      autobackup,
                      needed_vars);
        for hupa_stored in &hupas {
            if hupa_stored.get_name() == hupa.get_name() &&
               hupa_stored.get_category() == hupa.get_category() &&
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
