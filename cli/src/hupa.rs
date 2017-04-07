use DEFAULT_FSO;
use colored::*;
use humansize::FileSize;
use std::io::Write;
use libhupa::*;

/// Interface for actions
fn exec_hupa<F>(hupa: &Hupa, exec: F, print: &str)
    where F: FnOnce(&Hupa) -> Result<()>
{
    let mut stdout = ::std::io::stdout();
    write!(stdout,
           "{} {} ({})... ",
           print,
           hupa.get_name().yellow(),
           hupa.get_origin_size()
               .unwrap_or(0)
               .file_size(DEFAULT_FSO)
               .unwrap())
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
        exec_hupa(&hupa, |h| h.backup(), "Backing up");
    }
}

/// Restore hupas with interface
pub fn restore(hupas: &[Hupa]) {
    for hupa in hupas {
        exec_hupa(&hupa, |h| h.restore(), "Restoring");
    }
}

/// Clean hupas with interface
pub fn clean(hupas: &[Hupa]) {
    for hupa in hupas {
        exec_hupa(&hupa, |h| h.delete_backup(), "Cleaning");
    }
}
