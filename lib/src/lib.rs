//! Hupa is a tool to backup and restore some settings or file of your system

#![deny(missing_docs)]
#![recursion_limit="128"]

#[cfg(unix)]
extern crate libc;

extern crate app_dirs;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate json;

mod category;
mod config;
mod error;
mod fs_extra;
mod hupa;
mod metadata;
mod vars;

pub use category::*;
pub use config::*;
pub use error::*;
pub use hupa::*;
pub use metadata::*;
pub use vars::*;

use app_dirs::AppInfo;

/// `APP_INFO` is used for the crate `app_dirs` to get config dir, data dir and else.
pub const APP_INFO: AppInfo = AppInfo {
    name: "hupa",
    author: "fourdotfiveg",
};
