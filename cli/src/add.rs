use clap::ArgMatches;
use colored::*;
use io::*;
use libhupa::*;

/// Add subcommand
pub fn add_subcommand(hupas: &mut Vec<Hupa>, config: &Config, sub_m: &ArgMatches) {
    // TODO check if hupa is already used
    let count = sub_m
        .value_of("count")
        .unwrap_or("1")
        .parse::<usize>()
        .unwrap_or(1);
    for _ in 0..count {
        let name = read_line("Name: ");
        let desc = read_line("Description: ");
        let categories = read_line("Categories (ex: os/linux): ");
        let origin = read_line("Origin path: ");
                #[cfg(unix)]
        let origin = origin.replace('~', env!("HOME"));
        let autobackup = read_line_bool("Enable autobackup (y/n)? ", "The answer is yes or no");
        println!("{} is now added.", name.yellow());
        let hupa =
            Hupa::new(name,
                      desc,
                      categories.split('/').map(|s| s.to_string()).collect(),
                      Hupa::get_default_backup_parent().expect("Can't get default backup parent"),
                      origin,
                      autobackup);
        hupas.push(hupa);
    }
    save_hupas(&config, &hupas);

}
