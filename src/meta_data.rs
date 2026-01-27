use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A flexible value type for storing metadata.
///
/// `MetaValue` supports primitive types (integers, floats, strings),
/// as well as collections (vectors and maps) that can be nested arbitrarily.
///
/// # Examples
///
/// ```
/// use synapse_db::meta_data::MetaValue;
/// use std::collections::HashMap;
///
/// // Primitive values
/// let int_val = MetaValue::Int(42);
/// let str_val = MetaValue::Str("hello".to_string());
///
/// // Nested map
/// let mut map = HashMap::new();
/// map.insert("count".to_string(), MetaValue::Int(10));
/// let map_val = MetaValue::Map(map);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetaValue {
    /// A 64-bit signed integer value.
    Int(i64),
    /// A 64-bit floating point value.
    Float(f64),
    /// A UTF-8 string value.
    Str(String),
    /// A vector of `MetaValue` items, allowing heterogeneous collections.
    Vec(Vec<MetaValue>),
    /// A string-keyed map of `MetaValue` items, similar to a JSON object.
    Map(HashMap<String, MetaValue>),
}

/// A trait for types that have metadata storage.
///
/// Provides default implementations for `set_meta` and `get_meta` methods
/// that work with any type containing a `MetaValue` field.
pub trait HasMetadata {
    /// Returns a reference to the metadata.
    fn metadata(&self) -> &MetaValue;
    /// Returns a mutable reference to the metadata.
    fn metadata_mut(&mut self) -> &mut MetaValue;

    /// Sets a key-value pair in the metadata.
    ///
    /// If the current metadata is a `MetaValue::Map`, the key-value pair is inserted directly.
    /// If the metadata is a primitive value, it is first converted to a map with the
    /// original value stored under the key `"value"`, then the new key-value pair is added.
    fn set_meta(&mut self, key: &str, value: MetaValue) {
        if let MetaValue::Map(ref mut map) = self.metadata_mut() {
            map.insert(key.to_string(), value);
        } else {
            let mut map = HashMap::new();
            map.insert("value".to_string(), self.metadata().clone());
            map.insert(key.to_string(), value);
            *self.metadata_mut() = MetaValue::Map(map);
        }
    }

    /// Retrieves a value from the metadata by key.
    ///
    /// Returns `Some(&MetaValue)` if the metadata is a map and contains the key,
    /// otherwise returns `None`.
    fn get_meta(&self, key: &str) -> Option<&MetaValue> {
        if let MetaValue::Map(ref map) = self.metadata() {
            map.get(key)
        } else {
            None
        }
    }
}
