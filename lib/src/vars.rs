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

    /// Write to string
    pub fn write_to_string(&self, s: &mut String) {
        for var in &self.vars {
            s.push_str(&format!("{}={}\n", var.0, var.1));
        }
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

#[cfg(test)]
mod unit_tests {
    use super::*;
    use std::io::Cursor;

    fn set_of_var() -> Vec<Var> {
        vec![("Hello", true),
             ("World", true),
             ("he", false),
             ("yo", false)]
                .into_iter()
                .map(|s| (s.0.to_string(), s.1))
                .collect()
    }

    fn vars_string() -> String {
        let mut buf = String::new();
        for var in set_of_var() {
            buf.push_str(&format!("{}={}\n", var.0, var.1));
        }
        buf
    }

    #[test]
    fn read_from_buf() {
        let handler = VarsHandler::read_from_buf(vars_string()).unwrap();
        assert_eq!(handler.vars, set_of_var());
    }

    #[test]
    #[should_panic]
    fn read_from_buf_err() {
        let s = "hello\nhello=true";
        let handler = VarsHandler::read_from_buf(s).unwrap();
        // TODO impl iter
    }

    #[test]
    fn write_to_stream() {
        let handler = VarsHandler::new(set_of_var());
        let mut buf = Vec::new();
        handler.write_to_stream(&mut buf).unwrap();
        assert_eq!(String::from_utf8(buf).unwrap(), vars_string());
    }
}
