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
use fs_extra::{check_older, copy_dir, get_size};
use std::cmp::{Eq, PartialEq, PartialOrd, Ord, Ordering};
use std::fs;
use std::path::{Path, PathBuf};
use vars::VarsHandler;

/// Hupa is a class to handle a backup
///
/// # Arguments
///
/// `name` - Name of the hupa, can be whatever the user wants
///
/// `desc` - A small description of what the hupa is.
///
/// `category` - Category of the hupa. e.j: 'os', 'dotfiles', etc...
///
/// `backup_parent` - Parent directory of the hupa, where it will be stored
///
/// `origin_path` is the directory of the backed up directory
///
/// `autobackup` - Daemon specific variable, enable autobackup.
///
/// `needed_vars` - Vars needed to backup or restore this hupa
#[derive(Clone, Debug)]
pub struct Hupa {
    name: String,
    desc: String,
    category: Vec<String>,
    backup_parent: PathBuf,
    origin_path: PathBuf,
    autobackup: bool,
    needed_vars: Vec<String>,
}
// TODO replace path by string to allow vars

impl Hupa {
    /// Default constructor
    pub fn new<P: AsRef<Path>, Q: AsRef<Path>, S: AsRef<str>>(
        name: S,
        desc: S,
        category: Vec<String>,
        backup_parent: P,
        origin_path: Q,
        autobackup: bool,
        needed_vars: Vec<String>,
    ) -> Hupa {
        Hupa {
            name: name.as_ref().to_string(),
            desc: desc.as_ref().to_string(),
            category: category,
            backup_parent: backup_parent.as_ref().to_owned(),
            origin_path: origin_path.as_ref().to_path_buf(),
            autobackup: autobackup,
            needed_vars: needed_vars,
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

    /// Get category of this hupa
    pub fn get_category(&self) -> &Vec<String> {
        &self.category
    }

    /// Get category of this hupa in string format
    pub fn get_category_str(&self) -> String {
        let mut category = self.category
            .iter()
            .map(|s| format!("{}/", s))
            .collect::<String>();
        category.pop();
        category
    }

    /// Get needed vars
    pub fn get_needed_vars(&self) -> &Vec<String> {
        &self.needed_vars
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
    ///
    /// May fail when creating and moving new files
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

    /// Set category of the hupa
    ///
    /// May fail when creating and moving new files
    pub fn set_category(&mut self, category: Vec<String>) -> Result<()> {
        let old_backup_dir = self.backup_dir();
        self.category = category;
        let new_backup_dir = self.backup_dir();
        if old_backup_dir.exists() {
            move_all(old_backup_dir, new_backup_dir)?;
        }
        Ok(())
    }

    /// Set needed vars
    pub fn set_needed_vars(&mut self, needed_vars: Vec<String>) {
        self.needed_vars = needed_vars;
    }

    /// Set backup parent of the hupa
    ///
    /// May fail when creating and moving new files
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
        for sub_category in &self.category {
            hupas = hupas.join(sub_category);
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
        check_older(&self.origin_path, &backup)
    }

    /// Check if needed vars are activated
    fn vars_check(&self, vars_handler: &VarsHandler) -> Result<()> {
        for var in &self.needed_vars {
            match vars_handler.get_var(var) {
                Some(true) => {}
                _ => bail!(ErrorKind::MissingNeededVar(var.clone())),
            }
        }
        Ok(())
    }

    /// Backup hupa
    pub fn backup(&self, vars_handler: &VarsHandler) -> Result<()> {
        self.vars_check(vars_handler)?;
        let backup_dir = self.backup_dir();
        if !self.origin_path.exists() {
            bail!(ErrorKind::MissingOrigin(
                self.origin_path.display().to_string(),
            ));
        }
        if let Ok(false) = self.has_origin_changed() {
            return Ok(());
        }
        #[cfg(unix)] self.set_eid_backup()?;
        // TODO add file sync
        self.delete_backup()?;
        if let Some(p) = backup_dir.parent() {
            fs::create_dir_all(p)?;
        }
        copy_all(&self.origin_path, &backup_dir)?;
        Ok(())
    }

    /// Restore hupa
    pub fn restore(&self, vars_handler: &VarsHandler) -> Result<()> {
        self.vars_check(vars_handler)?;
        let backup_dir = self.backup_dir();
        if !backup_dir.exists() {
            bail!(ErrorKind::MissingBackup(backup_dir.display().to_string()));
        }
        #[cfg(unix)] self.set_eid_restore()?;
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

impl PartialEq<Hupa> for Hupa {
    fn eq(&self, other: &Hupa) -> bool {
        self.name == other.name && self.category == other.category
    }
}

impl Eq for Hupa {}

impl PartialOrd for Hupa {
    fn partial_cmp(&self, other: &Hupa) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hupa {
    fn cmp(&self, other: &Hupa) -> Ordering {
        match self.get_category_str().cmp(&other.get_category_str()) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => self.name.cmp(&other.name),
        }
    }
}

/// Copy file or directory to path
///
/// `from` - File or directory to copy
///
/// `to` - Destination path
fn copy_all<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<()> {
    let (from, to) = (from.as_ref(), to.as_ref());
    fs::create_dir_all(&to.parent().unwrap())?;
    if from.is_file() {
        fs::copy(from, to)?;
    } else if from.is_dir() {
        fs::create_dir_all(&to)?;
        copy_dir(from, to)?;
    }
    Ok(())
}

/// Move dir to new dir
fn move_all<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<()> {
    let (from, to) = (from.as_ref(), to.as_ref());
    copy_all(&from, to)?;
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

#[cfg(test)]
mod unit_tests {
    use super::*;

    fn set_of_hupas() -> Vec<Hupa> {
        vec![
            ("abc", vec!["test", "hello"]),
            ("def", vec!["test", "hello"]),
            ("ghi", vec!["test"]),
            ("jkl", vec!["test"]),
            ("mno", vec!["test"]),
        ].into_iter()
            .map(|(n, v)| {
                Hupa::new(
                    n,
                    "",
                    v.into_iter().map(|s| s.to_string()).collect(),
                    "/",
                    "/",
                    true,
                    Vec::new(),
                )
            })
            .collect()
    }

    #[test]
    fn hupa_category_str() {
        let hupas = set_of_hupas();
        assert_eq!(hupas[0].get_category_str(), "test/hello");
        assert_eq!(hupas[2].get_category_str(), "test");
    }

    #[test]
    fn hupa_eq_test() {
        let hupas = set_of_hupas();
        assert_eq!(hupas[0], hupas[0]);
        assert_ne!(hupas[1], hupas[2]);
    }

    #[test]
    fn hupa_ord_test() {
        let hupas = set_of_hupas();
        assert!(hupas[0] < hupas[1]);
        assert!(hupas[0] > hupas[2]);
        assert!(hupas[0] > hupas[3]);
        assert!(hupas[0] > hupas[4]);
    }
}
