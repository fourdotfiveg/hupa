//! Vars handler

use error::*;
use std::io::{Read, Write};

/// Var type, first arg is var name and second arg is var's state (enabled or disabled)
pub type Var = (String, bool);

/// Handle all vars
pub struct VarsHandler {
    vars: Vec<Var>,
}

impl VarsHandler {
    /// Default constructor
    pub fn new(vars: Vec<Var>) -> VarsHandler {
        VarsHandler { vars: vars }
    }

    /// Read from buffer
    pub fn read_from_buf<S: AsRef<str>>(buf: S) -> Result<VarsHandler> {
        let buf = buf.as_ref();
        let lines = buf.split('\n');
        let mut vars = Vec::new();
        for line in lines {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let mut split_eq = line.split('=');
            let name = split_eq.next().unwrap().to_string();
            let value = match split_eq.next() {
                Some(v) => v,
                None => bail!(ErrorKind::MissingValue(name)),
            };
            let value = match value.parse::<bool>() {
                Ok(v) => v,
                Err(_) => bail!(ErrorKind::InvalidValue(value.to_string())),
            };
            vars.push((name, value));
        }
        Ok(VarsHandler::new(vars))
    }

    /// Read from stream
    pub fn read_from_stream<R: Read>(stream: &mut R) -> Result<VarsHandler> {
        let mut buf = String::new();
        stream.read_to_string(&mut buf)?;
        Self::read_from_buf(buf)
    }

    /// Write to stream
    pub fn write_to_stream<W: Write>(&self, stream: &mut W) -> Result<()> {
        for var in &self.vars {
            write!(stream, "{}={}\n", var.0, var.1)?;
        }
        Ok(())
    }

    /// Add var
    pub fn add_var(&mut self, var: Var) {
        self.vars.push(var);
    }

    /// Remove var
    pub fn remove_var<S: AsRef<str>>(&mut self, var_name: S) -> Option<Var> {
        let var_name = var_name.as_ref();
        for i in 0..self.vars.len() {
            if self.vars[i].0 == var_name {
                return Some(self.vars.remove(i));
            }
        }
        None
    }
}
