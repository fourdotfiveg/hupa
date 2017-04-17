//! Group hupas by category

use hupa::Hupa;

/// Category is a struct containing hupas of the same category
#[derive(Clone, Debug)]
pub struct Category {
    name: String,
    hupas: Vec<Hupa>,
}

impl Category {
    /// Create a new category group
    pub fn new<S: AsRef<str>>(name: S) -> Category {
        Category {
            name: name.as_ref().to_string(),
            hupas: Vec::new(),
        }
    }
}
