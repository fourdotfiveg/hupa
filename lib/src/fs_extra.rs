//! Module to play with filesystem

use error::*;
use std::fs;
use std::path::Path;

/// Copy a directory
// TODO add overwrite and skip_exists
pub fn copy_dir<P: AsRef<Path>>(from: P, to: P) -> Result<u64> {
    let (from, to) = (from.as_ref(), to.as_ref());

    if !from.exists() {
        let err = ::std::io::Error::new(::std::io::ErrorKind::NotFound, "entity not found");
        return Err(err.into());
    }

    if !to.exists() {
        fs::create_dir(&to)?;
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
