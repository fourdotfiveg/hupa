//! Group hupas by category

use hupa::Hupa;
use std::cmp::PartialEq;
use std::iter::IntoIterator;
use std::slice::Iter;
use std::vec::IntoIter;

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

    /// Return the category name
    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// Return the hupas list
    pub fn get_hupas(&self) -> &Vec<Hupa> {
        &self.hupas
    }

    /// Insert a new hupa at index
    ///
    /// Return the hupa if category is not the same
    pub fn insert(&mut self, idx: usize, hupa: Hupa) -> Option<Hupa> {
        if hupa.get_category_str() == self.name {
            self.hupas.insert(idx, hupa);
            None
        } else {
            Some(hupa)
        }
    }

    /// Add a new hupa
    ///
    /// Return the hupa if category is not the same
    pub fn push(&mut self, hupa: Hupa) -> Option<Hupa> {
        if hupa.get_category_str() == self.name {
            self.hupas.push(hupa);
            None
        } else {
            Some(hupa)
        }
    }

    /// Remove hupa at index
    pub fn remove(&mut self, idx: usize) -> Hupa {
        self.hupas.remove(idx)
    }

    /// Remove the last hupa of the category
    pub fn pop(&mut self) -> Option<Hupa> {
        self.hupas.pop()
    }

    /// Return an iterator of hupas
    pub fn iter(&self) -> Iter<Hupa> {
        self.hupas.iter()
    }
}

impl PartialEq<Category> for Category {
    fn eq(&self, rhs: &Category) -> bool {
        self.name == rhs.name
    }
}

impl IntoIterator for Category {
    type Item = Hupa;
    type IntoIter = IntoIter<Hupa>;

    fn into_iter(self) -> IntoIter<Hupa> {
        self.hupas.into_iter()
    }
}
