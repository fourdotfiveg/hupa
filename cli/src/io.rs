use app_dirs;
use colored::*;
use libhupa;
use libhupa::*;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

/// Get metadata path
pub fn metadata_path() -> PathBuf {
    // TODO write path in config
    app_dirs::app_root(app_dirs::AppDataType::UserData, &APP_INFO)
        .unwrap()
        .join("metadata.json")
}

/// Read metadata
pub fn read_metadata_from_path(path: &PathBuf) -> Vec<Hupa> {
    let mut f = match File::open(path) {
        Ok(f) => f,
        Err(_) => return Vec::new(),
    };
    libhupa::read_metadata(&mut f, Some(libhupa::MetadataFormat::Json)).unwrap()
}

/// Read line
pub fn read_line(print: &str) -> String {
    let stdin = ::std::io::stdin();
    let mut stdout = ::std::io::stdout();
    let mut buf = String::new();
    while buf.is_empty() {
        stdout.write(print.as_bytes()).unwrap();
        stdout.flush().unwrap();
        stdin.read_line(&mut buf).unwrap();
        buf = buf.trim().to_string()
    }
    buf
}

/// Read line parse
pub fn read_line_parse<T: ::std::str::FromStr>(print: &str, err_msg: &str) -> T {
    loop {
        let readed = read_line(print);
        if let Ok(r) = readed.parse::<T>() {
            return r;
        } else {
            println!("{}", err_msg.red())
        }
    }
}

/// Save hupas
pub fn save_hupas(path: &PathBuf, hupas: &Vec<Hupa>) {
    let mut f = File::create(path).unwrap();
    libhupa::write_metadata(&mut f, &hupas, libhupa::MetadataFormat::Json).unwrap();
}
