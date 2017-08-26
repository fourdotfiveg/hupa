use DEFAULT_FSO;
use colored::*;
use humansize::FileSize;
use io::*;
use std::io::Write;
use libhupa::*;

/// Return list of hupa from `hupas_names`
pub fn resolve_names(hupas_names: &[String], hupas: &[Hupa]) -> Vec<Hupa> {
    let mut resolved = Vec::new();
    for hupa_name in hupas_names {
        let mut found = false;
        for hupa in hupas {
            if hupa.get_name() == hupa_name ||
               &format!("{}/{}", hupa.get_category_str(), hupa.get_name()) == hupa_name {
                resolved.push(hupa.clone());
                found = true;
                break;
            }
        }
        if !found {
            println!("Can't find hupa for name {}", hupa_name);
        }
    }
    resolved
}

pub enum PrintOrder {
    BackupToOrigin,
    OriginToBackup,
    BackupToNull,
}

/// Interface for actions
pub fn exec_hupa<F>(hupa: &Hupa, exec: F, size_order: &PrintOrder, print: &str)
    where F: FnOnce(&Hupa) -> Result<()>
{
    let mut stdout = ::std::io::stdout();
    let backup = hupa.get_backup_size()
        .unwrap_or(0)
        .file_size(DEFAULT_FSO)
        .expect("Error while showing file size");
    let origin = hupa.get_origin_size()
        .unwrap_or(0)
        .file_size(DEFAULT_FSO)
        .expect("Error while showing file size");

    let (first, second, first_str, second_str) = match *size_order {
        PrintOrder::BackupToOrigin => (backup, origin, "backup", "origin"),
        PrintOrder::OriginToBackup => (origin, backup, "origin", "backup"),
        PrintOrder::BackupToNull => {
            (backup,
             0
                 .file_size(DEFAULT_FSO)
                 .expect("Error while showing file size"),
             "backup",
             "void")
        }
    };
    writef!(stdout,
            "{} {} ({}: {} -> {}: {})... ",
            print,
            hupa.get_name().yellow(),
            first_str,
            first,
            second_str,
            second);
    match exec(hupa) {
        Ok(_) => {
            writef!(stdout, "{}", "OK!".green());
        }
        Err(e) => {
            write!(stdout, "{}", "Error: ".red()).expect("Can't write to stdout");
            writef!(stdout, "{}", e);
        }
    }
    writef!(stdout, "\n");
}

/// Select hupas
pub fn select_hupas(hupas: &[Hupa], print: &str) -> Vec<Hupa> {
    for (i, hupa) in hupas.iter().enumerate() {
        println!("[{}] {}: {}", i + 1, hupa.get_name(), hupa.get_desc());
    }
    println!("[{}] Cancel", hupas.len() + 1);
    let mut selected = Vec::new();
    let mut valid = false;
    while !valid {
        selected = Vec::new();
        valid = true;
        let idxs = read_line_usize(&format!("{} [1-{}]: ", print, hupas.len() + 1),
                                   false,
                                   hupas.len() + 1);
        for idx in idxs {
            if idx == 0 || idx > hupas.len() + 1 {
                println!("{} {}", idx.to_string().red(), " is not in the range".red());
                valid = false;
                break;
            } else if idx == hupas.len() + 1 {
                println!("Action cancelled");
                return Vec::new();
            } else {
                selected.push(hupas[idx - 1].clone());
            }
        }
    }
    selected
}
