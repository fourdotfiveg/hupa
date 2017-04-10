//! Metadata module is used to store and read metadata.
//!
//! Metadata file contains all info to instantiate hupas.

mod json;

use error::*;
use hupa::Hupa;
use std::io::{Read, Write};

/// File format to use for metadata.
pub enum MetadataFormat {
    /// Read and write metadata to json format
    Json,
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
        MetadataFormat::Json => ::json::stringify(hupas).as_bytes().to_vec(),
    };
    stream.write_all(&to_write)?;
    Ok(())
}
