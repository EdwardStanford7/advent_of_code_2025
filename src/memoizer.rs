/*
    Memoizer abstraction which uses pointers
    for faster hashing
*/

use std::collections::HashMap;

/// HashRef
/// Abstraction around &T that hashes like a *const T,
/// for faster memoization
/// Tracks the lifetime 'a as a parameter so that we can
/// memoize only during a valid lifetime
#[derive(Debug)]
pub struct HashRef<'a, T>
where
    T: ?Sized,
{
    pub _ref: &'a T,
}

impl <'a, T> Clone for HashRef<'a, T>
where
    T: ?Sized,
{
    fn clone(&self) -> Self {
        HashRef { _ref: self._ref }
    }
}

// impl<'a, T> Sized for HashRef<'a, T> where T: ?Sized {}

// We should override equality to pointer equality
impl<'a, T> PartialEq for HashRef<'a, T>
where
    T: ?Sized,
{
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self._ref, other._ref)
    }
}

impl<'a, T> Eq for HashRef<'a, T>
where
    T: ?Sized,
{}

impl<'a, T> std::hash::Hash for HashRef<'a, T>
where
    T: ?Sized,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let ptr = self._ref as *const T;
        ptr.hash(state);
    }
}

pub struct Memoizer<U, T> {
    table: HashMap<U, T>,
}

impl<U, T> Memoizer<U, T>
where
    U: std::hash::Hash + Eq + Clone,
    T: Clone,
{
    pub fn new() -> Self {
        Memoizer {
            table: HashMap::new(),
        }
    }

    pub fn get<'a, 'b>(&'b self, element: &'a U) -> Option<&'b T> {
        self.table.get(element)
    }

    pub fn insert<'b>(&'b mut self, element: U, value: T) {
        self.table.insert(element, value);
    }
}
