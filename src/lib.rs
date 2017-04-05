//! Hupa is a tool to backup and restore some settings or file of your system

#![deny(missing_docs)]

extern crate app_dirs;
extern crate copy_dir;
#[macro_use]
extern crate error_chain;
#[cfg(feature="text-json")]
#[macro_use]
extern crate json;

mod error;
mod hupa;
mod metadata;

pub use error::*;
pub use hupa::*;
pub use metadata::*;

use app_dirs::AppInfo;

/// APP_INFO is used for the crate `app_dirs` to get config dir, data dir and else.
pub const APP_INFO: AppInfo = AppInfo {
    name: "hupa",
    author: "notkild",
};
