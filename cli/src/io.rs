use colored::*;
use libhupa;
use libhupa::*;
use std::fs::File;
use std::io::Write;

/// Read line
pub fn read_line(print: &str, need_input: bool) -> String {
    let stdin = ::std::io::stdin();
    let mut stdout = ::std::io::stdout();
    let mut buf = String::new();
    while buf.is_empty() {
        writef!(stdout, "{}", print);
        stdin
            .read_line(&mut buf)
            .expect("Error while reading stdin");
        buf = buf.trim().to_string();
        if !need_input {
            return buf;
        }
    }
    buf
}

/// Read line and parse
pub fn read_line_parse<T: ::std::str::FromStr>(print: &str) -> T {
    loop {
        let readed = read_line(print, true);
        if let Ok(r) = readed.parse::<T>() {
            return r;
        } else {
            println!("{}", "Invalid input".red());
        }
    }
}

/// Read line bool
pub fn read_line_bool(print: &str) -> bool {
    loop {
        let readed = read_line(print, true);
        let readed = readed.to_lowercase();
        if readed == "yes" || readed == "y" || readed == "true" || readed == "1" {
            return true;
        } else if readed == "no" || readed == "n" || readed == "false" || readed == "0" {
            return false;
        } else {
            println!("{}", "Invalid boolean".red());
        }
    }
}

/// Read line numbers
pub fn read_line_usize(print: &str, need_input: bool, max: usize) -> Vec<usize> {
    let mut result = Vec::new();
    let mut valid = false;
    while !valid {
        let readed = read_line(print, need_input);
        result = Vec::new();
        valid = true;
        for s in readed.split_whitespace() {
            if s.contains("..") {
                let (mut first, mut second) = parse_range(s, max);
                if first < 1 || first > max || second < 1 || second > max {
                    println!("{}", "Out of range".red());
                    valid = false;
                    break;
                }
                if first > second {
                    ::std::mem::swap(&mut first, &mut second);
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
                    println!("{}", "Out of range".red());
                    valid = false;
                    break;
                }
                result.push(num);
            }
        }
    }
    result
}

/// Parse range
fn parse_range(s: &str, max: usize) -> (usize, usize) {
    let mut splitted = s.split("..");
    let first = splitted.next().unwrap_or("1");
    let max_str = max.to_string();
    let second = splitted.next().unwrap_or(&max_str);
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
pub fn save_hupas(config: &Config, hupas: &[Hupa]) {
    let mut f = File::create(&config.metadata_path).expect("Can't create metadata file");
    libhupa::write_metadata(&mut f, &hupas.to_vec()).expect("Can't write to metadata file");
}
