//! Metadata module is used to store and read metadata.
//!
//! Metadata file contains all info to instantiate hupas.

mod json;

use config::*;
use error::*;
use hupa::Hupa;
use std::fs::File;
use std::io::{Read, Write};
use std::str::FromStr;

/// File format to use for metadata.
#[derive(Clone, Debug, PartialEq)]
pub enum MetadataFormat {
    /// Read and write metadata to json format
    Json,
}

/// Convert str into `MetadataFormat`
impl FromStr for MetadataFormat {
    type Err = Error;
    fn from_str(s: &str) -> Result<MetadataFormat> {
        match s.as_ref() {
            "json" => Ok(MetadataFormat::Json),
            s => bail!(ErrorKind::InvalidMetadataFormat(s.to_string())),
        }
    }
}

/// Convert `MetadataFormat` into String
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
pub fn read_metadata<R: Read>(stream: &mut R,
                              format: &Option<MetadataFormat>)
                              -> Result<Vec<Hupa>> {
    let mut buffer = String::new();
    stream.read_to_string(&mut buffer)?;
    let hupas = match *format {
        Some(MetadataFormat::Json) => {
            let json = ::json::parse(&buffer)?;
            json::json_to_hupas(&json)?
        }
        None => {
            if let Ok(json) = ::json::parse(&buffer) {
                json::json_to_hupas(&json)?
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
    read_metadata(&mut f, &Some(config.metadata_format.clone()))
}

/// Write metadata to a stream
///
/// `stream` - Stream to write metadata
///
/// `hupas` - Hupas to write metadata
///
/// `format` - Define the format in which the metadata will be
pub fn write_metadata<W: Write>(stream: &mut W,
                                hupas: &[Hupa],
                                format: &MetadataFormat)
                                -> Result<()> {
    let to_write = match *format {
        MetadataFormat::Json => {
            ::json::stringify_pretty(hupas.to_vec(), 2)
                .as_bytes()
                .to_vec()
        }
    };
    stream.write_all(&to_write)?;
    Ok(())
}

#[cfg(test)]
mod unit_tests {
    use std::io::Cursor;
    use super::*;

    #[test]
    fn read_metadata_no_format() {
        let hupas = vec![Hupa::new("a", "b", vec!["hello".to_string()], "/", "/", false),
                         Hupa::new("c", "d", vec!["hello".to_string()], "/", "/", false),
                         Hupa::new("e", "f", vec!["hello".to_string()], "/", "/", false)];
        let json = ::json::stringify(hupas.clone());
        let mut cursor = Cursor::new(json);
        let readed_hupas = read_metadata(&mut cursor, None).unwrap();
        assert_eq!(hupas, readed_hupas);
    }
}
