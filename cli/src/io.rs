use colored::*;
use libhupa;
use libhupa::*;
use std::fs::File;
use std::io::Write;

/// Return config
pub fn get_config() -> Config {
    Config::read_config().unwrap_or(Config::default())
}

/// Read metadata
pub fn read_metadata(config: &Config) -> Vec<Hupa> {
    let mut f = match File::open(&config.metadata_path) {
        Ok(f) => f,
        Err(_) => return Vec::new(),
    };
    libhupa::read_metadata(&mut f, Some(config.metadata_format.clone())).unwrap()
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

/// Read line bool
pub fn read_line_bool(print: &str, err_msg: &str) -> bool {
    loop {
        let readed = read_line(print);
        let readed = readed.to_lowercase();
        if readed == "yes" || readed == "y" {
            return true;
        } else if readed == "no" || readed == "n" {
            return false;
        } else {
            println!("{}", err_msg.red());
        }
    }
}

/// Read line numbers
pub fn read_line_usize(print: &str, err_msg: &str, max: usize) -> Vec<usize> {
    'main: loop {
        let readed = read_line(print);
        let mut result = Vec::new();
        for s in readed.split_whitespace() {
            if s.contains("..") {
                let (mut first, mut second) = parse_range(s, max);
                if first < 1 || first > max || second < 1 || second > max {
                    println!("{}", err_msg.red());
                    continue 'main;
                }
                if first > second {
                    let tmp = first;
                    first = second;
                    second = tmp;
                }
                if second == max {
                    second = max - 1;
                }
                for i in first..(second + 1) {
                    result.push(i);
                }
            } else {
                let num = s.parse().unwrap_or(0);
                if num < 1 || num > max {
                    println!("{}", err_msg.red());
                    continue 'main;
                }
                result.push(num);
            }
        }
        return result;
    }
}

/// Parse range
fn parse_range(s: &str, max: usize) -> (usize, usize) {
    let mut splitted = s.split("..");
    let first = splitted.next().unwrap_or("1");
    let max_str = max.to_string();
    let second = splitted.next().unwrap_or(max_str.as_str());
    let first = parse_one(first, 1);
    let second = parse_one(second, max);
    (first, second)
}

/// Parse only one number
fn parse_one(s: &str, or: usize) -> usize {
    if s.is_empty() {
        return or;
    }
    s.parse().unwrap_or(0)
}

/// Save hupas
pub fn save_hupas(config: &Config, hupas: &Vec<Hupa>) {
    let mut f = File::create(&config.metadata_path).unwrap();
    libhupa::write_metadata(&mut f, &hupas, config.metadata_format.clone()).unwrap();
}
