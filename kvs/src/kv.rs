use std::collections::HashMap;

/// The 'KvStore' stores string key/value pairs.
///
/// key/value pairs are stored in a 'HashMap' in memory and not persisted to disk.
///


pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    /// Creates a `KvStore`.
    pub fn new() ->KvStore {
        KvStore {
            map: HashMap::new(),
        }
    }
    /// Sets the value of a string key to a string.
    ///
    /// If the key already exists, the previous value will be overwritten.
    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key,value);
    }
    /// Gets the string value of a given string key.
    ///
    /// Returns `None` if the given key does not exist.
    pub fn get(&self, key: String) -> Option<String> {
        self.map.get(&key).cloned()
    }

    /// Remove a given key.
    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}