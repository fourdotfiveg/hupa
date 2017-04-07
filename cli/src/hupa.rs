use DEFAULT_FSO;
use colored::*;
use humansize::FileSize;
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

enum HupaLocation {
    Backup,
    Origin,
}

/// Interface for actions
fn exec_hupa<F>(hupa: &Hupa, exec: F, size_of: HupaLocation, print: &str)
    where F: FnOnce(&Hupa) -> Result<()>
{
    let mut stdout = ::std::io::stdout();
    let size = match size_of {
            HupaLocation::Backup => hupa.get_backup_size(),
            HupaLocation::Origin => hupa.get_origin_size(),
        }
        .unwrap_or(0);
    write!(stdout,
           "{} {} ({})... ",
           print,
           hupa.get_name().yellow(),
           size.file_size(DEFAULT_FSO).unwrap())
            .unwrap();
    stdout.flush().unwrap();
    match exec(hupa) {
        Ok(_) => {
            write!(stdout, "{}", "OK!".green()).unwrap();
            stdout.flush().unwrap();
        }
        Err(e) => {
            write!(stdout, "{}", "Error: ".red()).unwrap();
            stdout.write(e.description().as_bytes()).unwrap();
            stdout.flush().unwrap();
        }
    }
    stdout.write(b"\n").unwrap();
    stdout.flush().unwrap();

}

/// Backup hupas with interface
pub fn backup(hupas: &[Hupa]) {
    for hupa in hupas {
        exec_hupa(&hupa, |h| h.backup(), HupaLocation::Origin, "Backing up");
    }
}

/// Restore hupas with interface
pub fn restore(hupas: &[Hupa]) {
    for hupa in hupas {
        exec_hupa(&hupa, |h| h.restore(), HupaLocation::Backup, "Restoring");
    }
}

/// Clean hupas with interface
pub fn clean(hupas: &[Hupa]) {
    for hupa in hupas {
        exec_hupa(&hupa,
                  |h| h.delete_backup(),
                  HupaLocation::Backup,
                  "Cleaning");
    }
}
