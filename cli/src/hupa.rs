use DEFAULT_FSO;
use colored::*;
use humansize::FileSize;
use io::*;
use std::io::Write;
use libhupa::*;

/// Return list of hupa from hupas_names
pub fn resolve_names(hupas_names: &[String], hupas: &[Hupa]) -> Vec<Hupa> {
    let mut resolved = Vec::new();
    for hupa_name in hupas_names {
        let mut found = false;
        for hupa in hupas {
            if hupa.get_name() == hupa_name {
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

enum PrintOrder {
    BackupToOrigin,
    OriginToBackup,
    BackupToNull,
}

/// Interface for actions
fn exec_hupa<F>(hupa: &Hupa, exec: F, size_order: PrintOrder, print: &str)
    where F: FnOnce(&Hupa) -> Result<()>
{
    let mut stdout = ::std::io::stdout();
    let backup = hupa.get_backup_size()
        .unwrap_or(0)
        .file_size(DEFAULT_FSO)
        .unwrap();
    let origin = hupa.get_origin_size()
        .unwrap_or(0)
        .file_size(DEFAULT_FSO)
        .unwrap();

    let (first, second, first_str, second_str) = match size_order {
        PrintOrder::BackupToOrigin => (backup, origin, "backup", "origin"),
        PrintOrder::OriginToBackup => (origin, backup, "origin", "backup"),
        PrintOrder::BackupToNull => (backup, 0.file_size(DEFAULT_FSO).unwrap(), "backup", "void"),
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
            write!(stdout, "{}", "Error: ".red()).unwrap();
            writef!(stdout, "{}", e.description());
        }
    }
    writef!(stdout, "\n");
}

/// Backup hupas with interface
pub fn backup(hupas: &[Hupa]) {
    for hupa in hupas {
        exec_hupa(&hupa,
                  |h| h.backup(),
                  PrintOrder::OriginToBackup,
                  "Backing up");
    }
}

/// Restore hupas with interface
pub fn restore(hupas: &[Hupa]) {
    for hupa in hupas {
        exec_hupa(&hupa,
                  |h| h.restore(),
                  PrintOrder::BackupToOrigin,
                  "Restoring");
    }
}

/// Clean hupas with interface
pub fn clean(hupas: &[Hupa]) {
    for hupa in hupas {
        exec_hupa(&hupa,
                  |h| h.delete_backup(),
                  PrintOrder::BackupToNull,
                  "Cleaning");
    }
}

/// Select hupas
pub fn select_hupas(hupas: &[Hupa], print: &str) -> Vec<Hupa> {
    let mut selected = Vec::new();
    for (i, hupa) in hupas.iter().enumerate() {
        println!("[{}] {}: {}", i + 1, hupa.get_name(), hupa.get_desc());
    }
    println!("[{}] Cancel", hupas.len() + 1);
    'main: loop {
        let idxs = read_line_usize(&format!("{} [1-{}]: ", print, hupas.len() + 1),
                                   &format!("You should enter a number between 1 and {}",
                                            hupas.len() + 1),
                                   hupas.len() + 1);
        for idx in idxs {
            if idx == 0 || idx > hupas.len() + 1 {
                println!("{} {}", idx.to_string().red(), " is not in the range".red());
                continue 'main;
            } else if idx == hupas.len() + 1 {
                println!("Action cancelled");
                return Vec::new();
            } else {
                selected.push(hupas[idx - 1].clone());
            }
        }
        return selected;
    }
}