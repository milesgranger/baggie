use std::collections::{HashMap, hash_map::Keys};
use std::any::Any;


/// struct for collecting values of any type with a string key
#[derive(Default)]
pub struct Baggie {
    data: HashMap<String, Box<Any>>
}

impl Baggie {

    /// Initialize an empty `Baggie`
    pub fn new() -> Self {
        Baggie::default()
    }

    /// Insert a value into the baggie.
    pub fn insert<T: 'static>(&mut self, key: &str, value: T) {
        let value = Box::new(value);
        self.data.insert(key.into(), value);
    }

    /// Get a reference to something in the baggie
    pub fn get<T: 'static>(&self, key: &str) -> Option<&T> {
        self.data.get(key)?.downcast_ref::<T>()
    }

    /// Get a mutable reference to something in the baggie
    pub fn get_mut<T: 'static>(&mut self, key: &str) -> Option<&mut T> {
        self.data.get_mut(key)?.downcast_mut::<T>()
    }

    /// An iterator visiting all keys in arbitrary order.
    pub fn keys(&self) -> Keys<'_, String, Box<dyn Any>> {
        self.data.keys()
    }

    /// Number of elements in the map
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Clear the map of all key value pairs; but maintains allocated memory
    pub fn clear(&mut self) {
        self.data.clear()
    }

    /// Determine if the Baggie is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns true if the map contains a value for the given key.
    pub fn contains_key(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    /// Remove a key-value pair from the Baggie by key.
    /// if the key value pair existed, the raw [`Box<dyn Any>`] value will be returned
    pub fn remove(&mut self, key: &str) -> Option<Box<dyn Any>> {
        self.data.remove(key)
    }

}
