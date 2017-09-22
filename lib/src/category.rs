//! Group hupas by category

use hupa::Hupa;
use std::cmp::{Eq, PartialEq, PartialOrd, Ord, Ordering};
use std::iter::IntoIterator;
use std::ops::*;
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

    /// Length of hupas' vec
    pub fn len(&self) -> usize {
        self.hupas.len()
    }

    /// Sort all hupas
    pub fn sort(&mut self) {
        self.hupas.sort();
    }

    /// Return an iterator of hupas
    pub fn iter(&self) -> Iter<Hupa> {
        self.hupas.iter()
    }
}

impl PartialEq<Category> for Category {
    fn eq(&self, other: &Category) -> bool {
        self.name == other.name
    }
}

impl Eq for Category {}

impl PartialOrd for Category {
    fn partial_cmp(&self, other: &Category) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Category {
    fn cmp(&self, other: &Category) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl IntoIterator for Category {
    type Item = Hupa;
    type IntoIter = IntoIter<Hupa>;

    fn into_iter(self) -> IntoIter<Hupa> {
        self.hupas.into_iter()
    }
}

impl<'a> IntoIterator for &'a Category {
    type Item = &'a Hupa;
    type IntoIter = ::std::slice::Iter<'a, Hupa>;

    fn into_iter(self) -> Self::IntoIter {
        self.hupas.iter()
    }
}

impl Index<usize> for Category {
    type Output = Hupa;

    fn index(&self, index: usize) -> &Hupa {
        &self.hupas[index]
    }
}

impl Index<Range<usize>> for Category {
    type Output = [Hupa];

    fn index(&self, index: Range<usize>) -> &[Hupa] {
        &self.hupas[index]
    }
}

impl Index<RangeTo<usize>> for Category {
    type Output = [Hupa];

    fn index(&self, index: RangeTo<usize>) -> &[Hupa] {
        &self.hupas[index]
    }
}

impl Index<RangeFrom<usize>> for Category {
    type Output = [Hupa];

    fn index(&self, index: RangeFrom<usize>) -> &[Hupa] {
        &self.hupas[index]
    }
}

impl Index<RangeFull> for Category {
    type Output = [Hupa];

    fn index(&self, index: RangeFull) -> &[Hupa] {
        &self.hupas[index]
    }
}

/// Conversion into categories
pub trait IntoCategories {
    /// Performs the conversion
    fn into_categories(self) -> Vec<Category>;
}

impl IntoCategories for Vec<Hupa> {
    fn into_categories(self) -> Vec<Category> {
        let mut categories: Vec<Category> = Vec::new();
        'main: for hupa in self {
            for category in &mut categories {
                if &hupa.get_category_str() == category.get_name() {
                    category.push(hupa);
                    continue 'main;
                }
            }
            let mut category = Category::new(hupa.get_category_str());
            category.push(hupa);
            categories.push(category);
        }
        categories
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    fn set_of_hupas() -> Vec<Hupa> {
        vec![
            ("abc", vec!["test", "hello"]),
            ("def", vec!["test", "hello"]),
            ("ghi", vec!["test"]),
            ("jkl", vec!["test"]),
            ("mno", vec!["test"]),
        ].into_iter()
            .map(|(n, v)| {
                Hupa::new(
                    n,
                    "",
                    v.into_iter().map(|s| s.to_string()).collect(),
                    "/",
                    "/",
                    true,
                    Vec::new(),
                )
            })
            .collect()
    }

    fn category_of_test() -> Category {
        let mut category = Category::new("test");
        for hupa in set_of_hupas() {
            category.push(hupa);
        }
        category
    }

    #[test]
    fn category_push_test() {
        let mut category = Category::new("test");
        for hupa in set_of_hupas() {
            category.push(hupa);
        }
        assert_eq!(category.len(), 3);
        assert_eq!(category[0], set_of_hupas()[2]);
    }

    #[test]
    fn category_pop_test() {
        let mut category = category_of_test();
        let mut hupas: Vec<Hupa> = set_of_hupas()
            .into_iter()
            .filter(|h| h.get_category_str() == "test")
            .collect();
        for _ in 0..2 {
            assert_eq!(category.pop(), hupas.pop());
        }
    }

    #[test]
    fn category_index_test() {
        let valid_hupas = set_of_hupas()[2..].to_vec();
        let category = category_of_test();
        for i in 0..2 {
            assert_eq!(valid_hupas[i], category[i]);
        }
    }

    #[test]
    fn hupas_into_categories() {
        let hupas = set_of_hupas();
        let categories = hupas.into_categories();
        assert_eq!(categories.len(), 2);
        assert_eq!(categories[0].get_name(), "test/hello");
        assert_eq!(categories[1].get_name(), "test");
    }
}
