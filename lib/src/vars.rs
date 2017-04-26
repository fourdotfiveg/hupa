//! Vars handler

use error::*;

/// Var type, first arg is var name and second arg is var's state (enabled or disabled)
pub type Var = (String, bool);

/// Handle all vars 
pub struct VarsHandler {
    vars: Vec<Var>
}

impl VarsHandler {
    /// Default constructor
    pub fn new(vars: Vec<Var>) -> VarsHandler {
        VarsHandler {
            vars: vars
        }
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

    /// Add var
    pub fn add_var(&mut self, var: Var) {
        self.vars.push(var);
    }

    /// Remove var
    pub fn remove_var<S: AsRef<str>>(&mut self, var_name: S) -> Option<Var> {
        let var_name = var_name.as_ref();
        for i in 0..self.vars.len() {
            if self.vars[i].0 == var_name {
                return Some(self.vars.remove(i))
            }
        }
        None
    }
}
