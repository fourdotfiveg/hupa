//! Hupa module is the core of the library.
//!
//! Each hupa can be considered as a backup.
//!
//! They contain a path to their backup and their origin.

#[cfg(unix)]
mod unix;
#[cfg(unix)]
pub use self::unix::*;

use APP_INFO;
use app_dirs::{self, AppDataType};
use error::*;
use fs_extra::{copy_dir, get_size};
use std::fs;
use std::path::{Path, PathBuf};

/// Hupa is a class to handle a backup
///
/// # Arguments
///
/// `name` - Name of the hupa, can be whatever the user wants
///
/// `desc` - A small description of what the hupa is.
///
/// `categories` - All the categories of the hupa. e.j: 'os', 'dotfiles', etc...
///
/// `origin_path` is the directory of the backed up directory
///
/// `autobackup` - Daemon specific variable, enable autobackup.
#[derive(Clone, Debug, PartialEq)]
pub struct Hupa {
    name: String,
    desc: String,
    categories: Vec<String>,
    origin_path: PathBuf,
    autobackup: bool,
}

impl Hupa {
    /// Default constructor
    pub fn new<P: AsRef<Path>, S: AsRef<str>>(name: S,
                                              desc: S,
                                              categories: Vec<String>,
                                              origin_path: P,
                                              autobackup: bool)
                                              -> Hupa {
        Hupa {
            name: name.as_ref().to_string(),
            desc: desc.as_ref().to_string(),
            categories: categories,
            origin_path: origin_path.as_ref().to_path_buf(),
            autobackup: autobackup,
        }
    }

    /// Get name of the hupa
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Get description of the hupa
    pub fn get_desc(&self) -> &str {
        &self.desc
    }

    /// Get categories of this hupa
    pub fn get_categories(&self) -> &Vec<String> {
        &self.categories
    }

    /// Get categories of this hupa in string format
    pub fn get_categories_str(&self) -> String {
        let mut categories = self.categories
            .iter()
            .map(|s| format!("{}/", s))
            .collect::<String>();
        categories.pop();
        categories
    }

    /// Get origin path of this hupa
    pub fn get_origin(&self) -> &PathBuf {
        &self.origin_path
    }

    /// Get autobackup state
    pub fn is_autobackup_enabled(&self) -> bool {
        self.autobackup
    }

    /// Return the backup directory of the hupa
    pub fn backup_dir(&self) -> Result<PathBuf> {
        let mut hupas = app_dirs::app_root(AppDataType::UserData, &APP_INFO)?;
        for category in &self.categories {
            hupas = hupas.join(category);
        }
        hupas = hupas.join(&self.name);
        Ok(hupas)
    }

    /// Get the backup size
    pub fn get_backup_size(&self) -> Result<u64> {
        get_size(self.backup_dir()?).map_err(|e| e.into())
    }

    /// Get the origin size
    pub fn get_origin_size(&self) -> Result<u64> {
        get_size(&self.origin_path).map_err(|e| e.into())
    }

    /// Check if origin has changed
    pub fn has_origin_changed(&self) -> Result<bool> {
        let origin_metadata = self.origin_path.metadata()?;
        let origin_time = origin_metadata.modified()?;
        let backup = self.backup_dir()?;
        let backup_metadata = backup.metadata()?;
        let backup_time = backup_metadata.modified()?;
        Ok(origin_time < backup_time)
    }

    /// Backup hupa
    pub fn backup(&self) -> Result<()> {
        let backup_dir = self.backup_dir()?;
        if !self.origin_path.exists() {
            bail!(ErrorKind::MissingOrigin(self.origin_path.display().to_string()));
        }
        if let Ok(b) = self.has_origin_changed() {
            if b {
                return Ok(());
            }
        }
        // TODO add overwrite and skip exist
        self.delete_backup()?;
        fs::create_dir_all(&backup_dir.parent().unwrap())?;
        copy_all(&self.origin_path, &backup_dir)?;
        Ok(())
    }

    /// Restore hupa
    pub fn restore(&self) -> Result<()> {
        // TODO restore_with_progress
        let backup_dir = self.backup_dir()?;
        if !backup_dir.exists() {
            bail!(ErrorKind::MissingBackup(backup_dir.display().to_string()));
        }
        // TODO add overwrite and skip exist
        self.delete_origin()?;
        copy_all(&backup_dir, &self.origin_path)?;
        Ok(())
    }

    /// Delete backup
    pub fn delete_backup(&self) -> Result<()> {
        let backup_dir = self.backup_dir()?;
        if backup_dir.exists() {
            remove_all(&backup_dir)?;
        }
        Ok(())
    }

    /// Delete origin
    pub fn delete_origin(&self) -> Result<()> {
        if self.origin_path.exists() {
            remove_all(&self.origin_path)?;
        }
        Ok(())
    }
}

/// Copy file or directory to path
///
/// `from` - File or directory to copy
///
/// `to` - Destination path
fn copy_all<P: AsRef<Path>>(from: P, to: P) -> Result<()> {
    let from = from.as_ref();
    if from.is_file() {
        fs::copy(from, to)?;
    } else if from.is_dir() {
        fs::create_dir_all(&to)?;
        copy_dir(from, to.as_ref())?;
    }
    Ok(())
}

/// Remove file or directory from `path`
///
/// `path` - path to the file or directory to remove
fn remove_all<P: AsRef<Path>>(path: P) -> Result<()> {
    let path = path.as_ref();
    if path.is_file() {
        fs::remove_file(&path)?;
    } else {
        fs::remove_dir_all(&path)?;
    }
    Ok(())
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    fn vec_categories() -> Vec<(String, Vec<String>)> {
        vec![("test", vec!["test"]),
             ("os", vec!["linux"]),
             ("os", vec!["osx"]),
             ("dotfiles", vec!["nvim"]),
             ("dotfiles", vec!["emacs"]),
             ("projects", vec!["c"]),
             ("projects", vec!["rust"])]
                .into_iter()
                .map(|(a, b)| (a.to_owned(), b.iter().map(|s| s.to_string()).collect()))
                .collect()
    }

    #[test]
    fn backup_dir_fn_test() {
        let app_dir = app_dirs::app_root(AppDataType::UserData, &APP_INFO).unwrap();
        let app_dir = app_dir.to_string_lossy();
        for (name, cat) in vec_categories() {
            let mut cat_str = cat.iter()
                .map(|s| format!("{}/", s))
                .collect::<String>();
            cat_str.pop();
            assert_eq!(Hupa::new(&name, &"".to_string(), cat.clone(), "/", false)
                           .backup_dir()
                           .unwrap()
                           .to_string_lossy(),
                       format!("{}/{}/{}", app_dir, cat_str, name));
        }
    }
}
