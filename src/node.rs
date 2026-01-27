use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::meta_data::{HasMetadata, MetaValue};

/// A node in the database hierarchy.
///
/// Each `Node` has a unique identifier, an optional parent reference,
/// a count of its children, a name, and flexible metadata storage.
///
/// # Examples
///
/// ```
/// use synapse_db::meta_data::MetaValue;
/// use synapse_db::node::Node;
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
}

impl HasMetadata for Node {
    fn metadata(&self) -> &MetaValue {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut MetaValue {
        &mut self.metadata
    }
}
