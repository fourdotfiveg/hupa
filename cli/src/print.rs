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
    for category in &categories {
        print_category(category, size_enabled);
    }
}

/// Print category
fn print_category(category: &Category, size_enabled: bool) {
    let (sizes, total_str) = compute_size(category, size_enabled);
    println!(
        "{}: {} item(s){}",
        category.get_name().bold(),
        category.len(),
        total_str
    );
    for (i, hupa) in category.iter().enumerate() {
        print_hupa(hupa, &sizes[i], size_enabled);
    }
}

/// Print hua
fn print_hupa(hupa: &Hupa, size: &str, size_enabled: bool) {
    println!(" -- {}:", hupa.get_name().yellow().bold());
    println!("   -- origin: {}", hupa.get_origin().display());
    if size_enabled {
        println!("   -- backup size: {}", size);
    }
    let autobackup = if hupa.is_autobackup_enabled() {
        format!("{}", "enabled".green())
    } else {
        format!("{}", "disabled".red())
    };
    println!("   -- autobackup is {}", autobackup);
    println!("   -- description: {}", hupa.get_desc());
    let needed_vars = hupa.get_needed_vars();
    if needed_vars.len() > 0 {
        println!(
            "   -- needed vars: {}",
            needed_vars
                .iter()
                .map(|s| format!("{} ", s))
                .collect::<String>()
        );
    }
}

/// Compute size
fn compute_size(category: &Category, size_enabled: bool) -> (Vec<String>, String) {
    let mut sizes = Vec::new();
    let mut total = 0;
    let mut total_str = String::new();
    if size_enabled {
        for hupa in category {
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
    (sizes, total_str)
}
