//! Metadata module is used to store and read metadata.
//!
//! Metadata file contains all info to instantiate hupas.

mod json;

use config::*;
use error::*;
use hupa::Hupa;
use std::fs::File;
use std::io::{Read, Write};

/// File format to use for metadata.
#[derive(Clone, Debug)]
pub enum MetadataFormat {
    /// Read and write metadata to json format
    Json,
}

impl MetadataFormat {
    /// Convert str into MetadataFormat
    pub fn from_str<S: AsRef<str>>(s: S) -> Result<MetadataFormat> {
        match s.as_ref() {
            "json" => Ok(MetadataFormat::Json),
            s => bail!(ErrorKind::InvalidMetadataFormat(s.to_string())),
        }
    }
}

/// Convert MetadataFormat into String
impl Into<String> for MetadataFormat {
    fn into(self) -> String {
        match self {
            MetadataFormat::Json => "json".to_string(),
        }
    }
}

/// Read metadata from stream
///
/// `stream` - Stream to read metadata
///
/// `format` - Define the format of the metadata, if sets to none, it will try all
/// possibilities
pub fn read_metadata<R: Read>(stream: &mut R, format: Option<MetadataFormat>) -> Result<Vec<Hupa>> {
    let mut buffer = String::new();
    stream.read_to_string(&mut buffer)?;
    let hupas = match format {
        Some(MetadataFormat::Json) => {
            let json = ::json::parse(&buffer)?;
            json::json_to_hupas(json)?
        }
        None => {
            if let Ok(json) = ::json::parse(&buffer) {
                json::json_to_hupas(json)?
            } else {
                Vec::new()
            }
        }
    };
    Ok(hupas)
}

/// Read metadata from config
///
/// `config` - A reference to config
pub fn read_metadata_from_config(config: &Config) -> Result<Vec<Hupa>> {
    let mut f = match File::open(&config.metadata_path) {
        Ok(f) => f,
        Err(_) => return Ok(Vec::new()),
    };
    read_metadata(&mut f, Some(config.metadata_format.clone()))
}

/// Write metadata to a stream
///
/// `stream` - Stream to write metadata
///
/// `hupas` - Hupas to write metadata
///
/// `format` - Define the format in which the metadata will be
pub fn write_metadata<W: Write>(stream: &mut W,
                                hupas: &Vec<Hupa>,
                                format: MetadataFormat)
                                -> Result<()> {
    let hupas = hupas.clone();
    let to_write = match format {
        MetadataFormat::Json => ::json::stringify_pretty(hupas, 2).as_bytes().to_vec(),
    };
    stream.write_all(&to_write)?;
    Ok(())
}
