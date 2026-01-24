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
/// use synapse_db::node::MetaValue;
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

/// A node in the database hierarchy.
///
/// Each `Node` has a unique identifier, an optional parent reference,
/// a count of its children, a name, and flexible metadata storage.
///
/// # Examples
///
/// ```
/// use synapse_db::node::{Node, MetaValue};
///
/// // Create a root node with no parent
/// let root = Node::new(1, None, 2, "root", None);
///
/// // Create a child node
/// let child = Node::new(2, Some(1), 0, "child", None);
///
/// // Create a node with metadata
/// let node_with_meta = Node::new(3, None, 0, "config", Some(MetaValue::Int(42)));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    /// Unique identifier for this node.
    pub id: i64,
    /// The ID of this node's parent, or `None` if this is a root node.
    pub parent_id: Option<i64>,
    /// The number of direct children this node has.
    pub children_count: u32,
    /// The name of this node.
    pub name: String,
    /// Flexible metadata storage for this node.
    pub metadata: MetaValue,
}

impl Node {
    /// Creates a new `Node` with the given properties.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for this node
    /// * `parent_id` - The ID of the parent node, or `None` for root nodes
    /// * `children_count` - Number of direct children this node has
    /// * `name` - The name of this node
    /// * `metadata` - Optional metadata; defaults to an empty `MetaValue::Map` if `None`
    ///
    /// # Examples
    ///
    /// ```
    /// use synapse_db::node::Node;
    ///
    /// let node = Node::new(1, None, 0, "example", None);
    /// assert_eq!(node.id, 1);
    /// assert!(node.parent_id.is_none());
    /// ```
    pub fn new(
        id: i64,
        parent_id: Option<i64>,
        children_count: u32,
        name: &str,
        metadata: Option<MetaValue>,
    ) -> Self {
        Node {
            id,
            parent_id,
            children_count,
            name: name.to_string(),
            metadata: metadata.unwrap_or(MetaValue::Map(HashMap::new())),
        }
    }

    /// Sets a key-value pair in the node's metadata.
    ///
    /// If the current metadata is a `MetaValue::Map`, the key-value pair is inserted directly.
    /// If the metadata is a primitive value, it is first converted to a map with the
    /// original value stored under the key `"value"`, then the new key-value pair is added.
    ///
    /// # Arguments
    ///
    /// * `key` - The key under which to store the value
    /// * `value` - The `MetaValue` to store
    ///
    /// # Examples
    ///
    /// ```
    /// use synapse_db::node::{Node, MetaValue};
    ///
    /// let mut node = Node::new(1, None, 0, "example", None);
    /// node.set_meta("version", MetaValue::Int(1));
    /// ```
    pub fn set_meta(&mut self, key: &str, value: MetaValue) {
        if let MetaValue::Map(ref mut map) = self.metadata {
            map.insert(key.to_string(), value);
        } else {
            // Convert existing primitive metadata to a Map, preserving the old value
            let mut map = HashMap::new();
            map.insert("value".to_string(), self.metadata.clone());
            map.insert(key.to_string(), value);
            self.metadata = MetaValue::Map(map);
        }
    }

    /// Retrieves a value from the node's metadata by key.
    ///
    /// Returns `Some(&MetaValue)` if the metadata is a map and contains the key,
    /// otherwise returns `None`.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to look up
    ///
    /// # Examples
    ///
    /// ```
    /// use synapse_db::node::{Node, MetaValue};
    ///
    /// let mut node = Node::new(1, None, 0, "example", None);
    /// node.set_meta("count", MetaValue::Int(42));
    ///
    /// if let Some(MetaValue::Int(count)) = node.get_meta("count") {
    ///     assert_eq!(*count, 42);
    /// }
    /// ```
    pub fn get_meta(&self, key: &str) -> Option<&MetaValue> {
        if let MetaValue::Map(ref map) = self.metadata {
            map.get(key)
        } else {
            None
        }
    }
}
