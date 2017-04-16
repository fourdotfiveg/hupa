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
/// `backup_parent` - Parent directory of the hupa, where it will be stored
///
/// `origin_path` is the directory of the backed up directory
///
/// `autobackup` - Daemon specific variable, enable autobackup.
#[derive(Clone, Debug, PartialEq)]
pub struct Hupa {
    name: String,
    desc: String,
    categories: Vec<String>,
    backup_parent: PathBuf,
    origin_path: PathBuf,
    autobackup: bool,
}

impl Hupa {
    /// Default constructor
    pub fn new<P: AsRef<Path>, Q: AsRef<Path>, S: AsRef<str>>(name: S,
                                                              desc: S,
                                                              categories: Vec<String>,
                                                              backup_parent: P,
                                                              origin_path: Q,
                                                              autobackup: bool)
                                                              -> Hupa {
        Hupa {
            name: name.as_ref().to_string(),
            desc: desc.as_ref().to_string(),
            categories: categories,
            backup_parent: backup_parent.as_ref().to_owned(),
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

    /// Get the default backup parent
    pub fn get_default_backup_parent() -> Result<PathBuf> {
        ::app_dirs::app_root(::app_dirs::AppDataType::UserData, &APP_INFO).map_err(|e| e.into())
    }

    /// Get backup parent path
    pub fn get_backup_parent(&self) -> &PathBuf {
        &self.backup_parent
    }

    /// Get origin path of this hupa
    pub fn get_origin(&self) -> &PathBuf {
        &self.origin_path
    }

    /// Get autobackup state
    pub fn is_autobackup_enabled(&self) -> bool {
        self.autobackup
    }

    /// Set name of the hupa
    pub fn set_name(&mut self, name: String) -> Result<()> {
        let old_backup_dir = self.backup_dir();
        self.name = name;
        let new_backup_dir = self.backup_dir();
        if old_backup_dir.exists() {
            move_all(old_backup_dir, new_backup_dir)?;
        }
        Ok(())
    }

    /// Set description of the hupa
    pub fn set_desc(&mut self, desc: String) {
        self.desc = desc;
    }

    /// Set categories of the hupa
    pub fn set_categories(&mut self, categories: Vec<String>) -> Result<()> {
        let old_backup_dir = self.backup_dir();
        self.categories = categories;
        let new_backup_dir = self.backup_dir();
        if old_backup_dir.exists() {
            move_all(old_backup_dir, new_backup_dir)?;
        }
        Ok(())
    }

    /// Set backup parent of the hupa
    pub fn set_backup_parent<P: AsRef<Path>>(&mut self, backup_parent: P) -> Result<()> {
        let old_backup_dir = self.backup_dir();
        self.backup_parent = backup_parent.as_ref().to_path_buf();
        let new_backup_dir = self.backup_dir();
        if old_backup_dir.exists() {
            move_all(old_backup_dir, new_backup_dir)?;
        }
        Ok(())
    }

    /// Set origin path of the hupa
    pub fn set_origin_path<P: AsRef<Path>>(&mut self, origin_path: P) {
        self.origin_path = origin_path.as_ref().to_path_buf();
    }

    /// Set autobackup state
    pub fn set_autobackup(&mut self, autobackup: bool) {
        self.autobackup = autobackup;
    }

    /// Return the backup directory of the hupa
    pub fn backup_dir(&self) -> PathBuf {
        let mut hupas = self.backup_parent.clone();
        for category in &self.categories {
            hupas = hupas.join(category);
        }
        hupas = hupas.join(&self.name);
        hupas
    }

    /// Get the backup size
    pub fn get_backup_size(&self) -> Result<u64> {
        get_size(self.backup_dir()).map_err(|e| e.into())
    }

    /// Get the origin size
    pub fn get_origin_size(&self) -> Result<u64> {
        get_size(&self.origin_path).map_err(|e| e.into())
    }

    /// Check if origin has changed
    pub fn has_origin_changed(&self) -> Result<bool> {
        let backup = self.backup_dir();
        if get_size(&backup)? != get_size(&self.origin_path)? {
            return Ok(true);
        }
        let origin_metadata = self.origin_path.metadata()?;
        let origin_time = origin_metadata.modified()?;
        let backup_metadata = backup.metadata()?;
        let backup_time = backup_metadata.modified()?;
        Ok(origin_time < backup_time)
    }

    /// Backup hupa
    pub fn backup(&self) -> Result<()> {
        let backup_dir = self.backup_dir();
        if !self.origin_path.exists() {
            bail!(ErrorKind::MissingOrigin(self.origin_path.display().to_string()));
        }
        if let Ok(b) = self.has_origin_changed() {
            if b {
                return Ok(());
            }
        }
        #[cfg(unix)]
        self.set_eid_backup()?;
        // TODO add file sync
        self.delete_backup()?;
        if let Some(p) = backup_dir.parent() {
            fs::create_dir_all(p)?;
        }
        copy_all(&self.origin_path, &backup_dir)?;
        Ok(())
    }

    /// Restore hupa
    pub fn restore(&self) -> Result<()> {
        let backup_dir = self.backup_dir();
        if !backup_dir.exists() {
            bail!(ErrorKind::MissingBackup(backup_dir.display().to_string()));
        }
        #[cfg(unix)]
        self.set_eid_restore()?;
        // TODO add file sync
        self.delete_origin()?;
        if let Some(p) = self.origin_path.parent() {
            fs::create_dir_all(p)?;
        }
        copy_all(&backup_dir, &self.origin_path)?;
        Ok(())
    }

    /// Delete backup
    pub fn delete_backup(&self) -> Result<()> {
        let backup_dir = self.backup_dir();
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
fn copy_all<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<()> {
    let from = from.as_ref();
    if from.is_file() {
        fs::copy(from, to)?;
    } else if from.is_dir() {
        fs::create_dir_all(&to)?;
        copy_dir(from, to.as_ref())?;
    }
    Ok(())
}

/// Move dir to new dir
fn move_all<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<()> {
    let (from, to) = (from.as_ref(), to.as_ref());
    copy_dir(&from, to)?;
    if from.is_dir() {
        fs::remove_dir_all(from)?;
    } else if from.is_file() {
        fs::remove_file(from)?;
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
