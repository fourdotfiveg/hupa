//! Module to play with filesystem

use error::*;
use std::fs;
use std::path::Path;

/// Copy a directory
pub fn copy_dir<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<u64> {
    let (from, to) = (from.as_ref(), to.as_ref());

    if !from.exists() {
        let err = ::std::io::Error::new(::std::io::ErrorKind::NotFound, "entity not found");
        return Err(err.into());
    }

    if !to.exists() {
        fs::create_dir_all(&to)?;
    }

    let mut to = to.to_path_buf();
    let mut result = 0;
    for entry in fs::read_dir(&from)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = path.file_name().unwrap();
        to.push(file_name);
        if path.is_dir() {
            result += copy_dir(path.clone(), to.clone())?;
        } else if path.is_file() {
            result += fs::copy(&path, &to)?;
        }
        to.pop();
    }
    Ok(result)
}

/// Get size of file or directory
pub fn get_size<P: AsRef<Path>>(path: P) -> Result<u64> {
    let path = path.as_ref();
    if !path.exists() {
        let err = ::std::io::Error::new(::std::io::ErrorKind::NotFound, "entity not found");
        return Err(err.into());
    }
    let mut result = 0;
    if path.is_file() {
        result += path.metadata()?.len();
    } else if path.is_dir() {
        for entry in fs::read_dir(&path)? {
            let entry = entry?;
            let path = entry.path();
            result += get_size(&path)?;
        }
    }
    Ok(result)
}

/// Check if directory is older than source
pub fn check_older<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dir: Q) -> Result<bool> {
    let (src, dir) = (src.as_ref(), dir.as_ref());
    if !src.exists() || !dir.exists() {
        return Ok(true);
    }
    if src.metadata()?.modified()? > dir.metadata()?.modified()? {
        return Ok(true);
    }
    for entry in src.read_dir()? {
        let entry = entry?.path();
        let dir_entry = dir.join(entry.file_name().unwrap());
        if !dir_entry.exists() {
            return Ok(true);
        }
        if entry.is_dir() {
            if let Ok(true) = check_older(entry, dir_entry) {
                return Ok(true);
            }
        } else {
            let entry_met = entry.metadata()?;
            let dir_entry_met = dir_entry.metadata()?;
            if entry_met.modified()? > dir_entry_met.modified()? {
                return Ok(true);
            }
        }
    }
    Ok(false)
}
