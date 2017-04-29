//! Metadata module is used to store and read metadata.
//!
//! Metadata file contains all info to instantiate hupas.

mod json;

use config::*;
use error::*;
use hupa::Hupa;
use std::fs::File;
use std::io::{Read, Write};

/// Read metadata from stream
///
/// `stream` - Stream to read metadata
pub fn read_metadata<R: Read>(stream: &mut R) -> Result<Vec<Hupa>> {
    let mut buffer = String::new();
    stream.read_to_string(&mut buffer)?;
    let json = ::json::parse(&buffer)?;
    Ok(json::json_to_hupas(&json)?)
}

/// Read metadata from config
///
/// `config` - A reference to config
pub fn read_metadata_from_config(config: &Config) -> Result<Vec<Hupa>> {
    let mut f = match File::open(&config.metadata_path) {
        Ok(f) => f,
        Err(_) => return Ok(Vec::new()),
    };
    read_metadata(&mut f)
}

/// Write metadata to a stream
///
/// `stream` - Stream to write metadata
///
/// `hupas` - Hupas to write metadata
pub fn write_metadata<W: Write>(stream: &mut W, hupas: &[Hupa]) -> Result<()> {
    let to_write = ::json::stringify_pretty(hupas.to_vec(), 2)
        .as_bytes()
        .to_vec();
    stream.write_all(&to_write)?;
    Ok(())
}

#[cfg(test)]
mod unit_tests {
    use std::io::Cursor;
    use super::*;

    #[test]
    fn read_metadata_test() {
        let hupas = vec![Hupa::new("a",
                                   "b",
                                   vec!["hello".to_string()],
                                   "/",
                                   "/",
                                   false,
                                   Vec::new()),
                         Hupa::new("c",
                                   "d",
                                   vec!["hello".to_string()],
                                   "/",
                                   "/",
                                   false,
                                   Vec::new()),
                         Hupa::new("e",
                                   "f",
                                   vec!["hello".to_string()],
                                   "/",
                                   "/",
                                   false,
                                   Vec::new())];
        let json = ::json::stringify(hupas.clone());
        let mut cursor = Cursor::new(json);
        let readed_hupas = read_metadata(&mut cursor).unwrap();
        assert_eq!(hupas, readed_hupas);
    }
}
