//! Configuration module, read and write config file.

use APP_INFO;
use error::*;
use json::JsonValue;
use metadata::MetadataFormat;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

/// Configuration structure to read and write config.
#[derive(Clone, Debug)]
pub struct Config {
    /// Path to the metadata
    pub metadata_path: PathBuf,
    /// Format of the metadata
    pub metadata_format: MetadataFormat,
    /// Interval between each autobackup
    pub autobackup_interval: usize,
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

    /// Path to the config
    pub fn config_path() -> Result<PathBuf> {
        Ok(::app_dirs::app_root(::app_dirs::AppDataType::UserConfig, &APP_INFO)?
               .join("config.json"))
    }

    /// Read config from readable stream
    pub fn from_json_stream<R: Read>(stream: &mut R) -> Result<Config> {
        let mut buf = String::new();
        stream.read_to_string(&mut buf)?;
        let json = ::json::parse(&buf)?;
        // TODO better error handling
        let metadata_path = json["metadata_path"].as_str().unwrap();
        let metadata_format = json["metadata_format"].as_str().unwrap();
        let autobackup_interval = json["autobackup_interval"].as_usize().unwrap();
        Ok(Config::new(metadata_path,
                       MetadataFormat::from_str(metadata_format)?,
                       autobackup_interval))
    }

    /// Read config from user config
    pub fn read_config() -> Result<Config> {
        let path = Config::config_path()?;
        let mut f = File::open(&path)?;
        Config::from_json_stream(&mut f)
    }

    /// Save config
    pub fn save_config(&self) -> Result<()> {
        let path = Config::config_path()?;
        let mut f = File::create(&path)?;
        let json: JsonValue = self.clone().into();
        let json_str = ::json::stringify_pretty(json, 2);
        f.write(json_str.as_bytes())?;
        Ok(())
    }
}

/// Convert Config into Json
impl Into<JsonValue> for Config {
    fn into(self) -> JsonValue {
        let metadata_format: String = self.metadata_format.into();
        object!{
            "metadata_path" => self.metadata_path.display().to_string(),
            "metadata_format" => metadata_format,
            "autobackup_interval" => self.autobackup_interval
        }
    }
}

/// Default config, may be panic
impl Default for Config {
    fn default() -> Config {
        let metadata_path = ::app_dirs::app_root(::app_dirs::AppDataType::UserData, &APP_INFO)
            .unwrap()
            .join("metadata.json");
        Config::new(metadata_path, MetadataFormat::Json, 3600)
    }
}
