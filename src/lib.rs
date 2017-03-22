//! Hupa is a tool to backup and restore some settings or file of your system

#![deny(missing_docs)]

use std::path::{Path, PathBuf};

/// Hupa is a class to handle a backup
///
/// # Arguments
///
/// `backup_dir` is the directory of the backup
/// `restore_dir` is the director of the backed up directory
pub struct Hupa {
    backup_dir: PathBuf,
    restore_dir: PathBuf,
}

impl Hupa {
    /// Default constructor
    pub fn new<P: AsRef<Path>>(backup_dir: P, restore_dir: P) -> Hupa {
        Hupa {
            backup_dir: backup_dir.as_ref().to_path_buf(),
            restore_dir: restore_dir.as_ref().to_path_buf(),
        }
    }
}
