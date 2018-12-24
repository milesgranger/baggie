use std::collections::{HashMap, hash_map::Keys};
use std::any::Any;
use std::hash::Hash;
use std::borrow::Borrow;


/// struct for collecting values of any type with a string key
#[derive(Default)]
pub struct Baggie<K>
    where K: Eq + Hash
{
    data: HashMap<K, Box<Any>>
}

impl<K> Baggie<K>
    where K: Eq + Hash + Default
{

    /// Initialize an empty `Baggie`
    pub fn new() -> Self {
        Default::default()
    }

    /// Insert a value into the baggie.
    pub fn insert<T: 'static>(&mut self, key: K, value: T) {
        let value = Box::new(value);
        self.data.insert(key.into(), value);
    }

    /// Get a reference to something in the baggie
    pub fn get<T: 'static, Q: ?Sized>(&self, key: &Q) -> Option<&T>
        where K: Borrow<Q>,
              Q: Eq + Hash
    {
        self.data.get(key)?.downcast_ref::<T>()
    }

    /// Get a mutable reference to something in the baggie
    pub fn get_mut<T: 'static, Q: ?Sized>(&mut self, key: &Q) -> Option<&mut T>
        where K: Borrow<Q>,
              Q: Eq + Hash
    {
        self.data.get_mut(key)?.downcast_mut::<T>()
    }

    /// An iterator visiting all keys in arbitrary order.
    pub fn keys(&self) -> Keys<'_, K, Box<dyn Any>> {
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
    pub fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool
        where K: Borrow<Q>,
              Q: Eq + Hash
    {
        self.data.contains_key(key)
    }

    /// Remove a key-value pair from the Baggie by key.
    /// if the key value pair existed, the raw [`Box<dyn Any>`] value will be returned
    pub fn remove<Q: ?Sized>(&mut self, key: &Q) -> Option<Box<dyn Any>>
        where K: Borrow<Q>,
              Q: Eq + Hash
    {
        self.data.remove(key)
    }

}
