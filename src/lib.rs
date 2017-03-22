//! Hupa is a tool to backup and restore some settings or file of your system

#![deny(missing_docs)]

extern crate app_dirs;
#[macro_use]
extern crate error_chain;

mod error;

pub use error::*;

use app_dirs::{AppInfo, AppDataType};

/// APP_INFO is used for the crate `app_dirs` to get config dir, data dir and else.
pub const APP_INFO: AppInfo = AppInfo {
    name: "hupa",
    author: "notkild",
};

use std::path::{Path, PathBuf};

/// Hupa is a class to handle a backup
///
/// # Arguments
///
/// `category` is the category of the hupa, i.e category can be os, dotfiles or else
///
/// `sub_category` is the sub category of the hupa, i.e if category is os, sub_category can be
/// gentoo, fedora or windows
///
/// `restore_dir` is the director of the backed up directory
pub struct Hupa {
    category: String,
    sub_category: String,
    restore_dir: PathBuf,
}

impl Hupa {
    /// Default constructor
    pub fn new<P: AsRef<Path>, S: AsRef<str>>(category: S,
                                              sub_category: S,
                                              restore_dir: P)
                                              -> Hupa {
        Hupa {
            category: category.as_ref().to_owned(),
            sub_category: sub_category.as_ref().to_owned(),
            restore_dir: restore_dir.as_ref().to_path_buf(),
        }
    }

    /// Return the backup directory of the hupa
    pub fn backup_dir(&self) -> Result<PathBuf> {
        let hupas = app_dirs::app_dir(AppDataType::UserData, &APP_INFO, "hupas")?;
        let hupas = hupas.join(&self.category).join(&self.sub_category);
        Ok(hupas)
    }
}
