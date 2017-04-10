//! Configuration module, read and write config file.

use metadata::MetadataFormat;
use std::path::{Path, PathBuf};

/// Configuration structure to read and write config.
///
/// `metadata_path` - Path to the metadata
///
/// `metadata_format` - Format of the metadata
///
/// `autobackup_interval` - Interval between each autobackup
pub struct Config {
    metadata_path: PathBuf,
    metadata_format: MetadataFormat,
    autobackup_interval: usize,
}

impl Config {
    /// Default constructor
    pub fn new<P: AsRef<Path>>(metadata_path: P,
                               metadata_format: MetadataFormat,
                               autobackup_interval: usize)
                               -> Config {
        Config {
            metadata_path: metadata_path.as_ref().to_path_buf(),
            metadata_format: metadata_format,
            autobackup_interval: autobackup_interval,
        }
    }
}
