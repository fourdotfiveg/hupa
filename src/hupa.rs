//! Hupa module is the core of the library.
//!
//! Each hupa can be considered as a backup.
//!
//! They contain a path to their backup and their origin.

use APP_INFO;
use app_dirs::{self, AppDataType};
use error::*;
use std::fs;
use std::path::{Path, PathBuf};

/// Hupa is a class to handle a backup
///
/// # Arguments
///
/// `category` is the category of the hupa, i.e category can be os, dotfiles or else
///
/// `sub_categories` is the sub category of the hupa, i.e if category is os, sub_categories can be
/// gentoo, fedora or windows
///
/// `origin_path` is the director of the backed up directory
#[derive(Clone, Debug, PartialEq)]
pub struct Hupa {
    category: String,
    sub_categories: Vec<String>,
    origin_path: PathBuf,
}

impl Hupa {
    /// Default constructor
    pub fn new<P: AsRef<Path>, S: AsRef<str>>(category: S,
                                              sub_categories: &Vec<String>,
                                              origin_path: P)
                                              -> Hupa {
        Hupa {
            category: category.as_ref().to_owned(),
            sub_categories: sub_categories.clone(),
            origin_path: origin_path.as_ref().to_path_buf(),
        }
    }

    /// Get category of this hupa
    pub fn get_category(&self) -> &str {
        &self.category
    }

    /// Get sub categories of this hupa
    pub fn get_sub_categories(&self) -> &Vec<String> {
        &self.sub_categories
    }

    /// Get origin path of this hupa
    pub fn get_origin(&self) -> &PathBuf {
        &self.origin_path
    }

    /// Return the backup directory of the hupa
    pub fn backup_dir(&self) -> Result<PathBuf> {
        let mut hupas = app_dirs::app_dir(AppDataType::UserData, &APP_INFO, "hupas")?;
        hupas = hupas.join(&self.category);
        for sub_category in &self.sub_categories {
            hupas = hupas.join(sub_category);
        }
        Ok(hupas)
    }

    /// Backup hupa
    pub fn backup(&self) -> Result<()> {
        let backup_dir = self.backup_dir()?;
        if !self.origin_path.exists() {
            // TODO return error
            return Ok(());
        }
        self.delete_backup()?;
        fs::create_dir_all(&backup_dir)?;
        fs::copy(&self.origin_path, &backup_dir)?;
        Ok(())
    }

    /// Restore hupa
    pub fn restore(&self) -> Result<()> {
        let backup_dir = self.backup_dir()?;
        if !backup_dir.exists() {
            // TODO return error
            return Ok(());
        }
        self.delete_origin()?;
        fs::copy(&backup_dir, &self.origin_path)?;
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
        let app_dir = app_dirs::app_dir(AppDataType::UserData, &APP_INFO, "hupas").unwrap();
        let app_dir = app_dir.to_string_lossy();
        for (cat, sub) in vec_categories() {
            let mut sub_str = sub.iter().map(|s| format!("{}/", s)).collect::<String>();
            sub_str.pop();
            assert_eq!(Hupa::new(&cat, &sub, "/").backup_dir().unwrap().to_string_lossy(),
                       format!("{}/{}/{}", app_dir, cat, sub_str));
        }
    }
}
