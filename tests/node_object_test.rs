//! Unit tests for the `Node` struct.
//!
//! These tests verify node creation, parent-child relationships,
//! and metadata handling functionality.

use std::collections::HashMap;
use synapse_db::meta_data::MetaValue;
use synapse_db::node::Node;

/// Tests that a node without a parent is initialized correctly.
#[test]
fn test_node_no_parent_init() {
    let node = Node::new(1, None, 0, "test", None);

    assert_eq!(node.id, 1);
    assert_eq!(node.children_count, 0);
    assert_eq!(node.name, "test");
    assert!(node.parent_id.is_none());
}

/// Tests that a node with a parent reference is initialized correctly.
#[test]
fn test_node_parent_init() {
    let parent = Node::new(1, None, 0, "test", None);
    let node = Node::new(2, Some(parent.id), 0, "test", None);

    assert_eq!(node.id, 2);
    assert_eq!(node.children_count, 0);
    assert_eq!(node.name, "test");
    assert_eq!(node.parent_id, Some(1));
}

/// Tests that a node created with `None` metadata defaults to an empty map.
#[test]
fn test_node_default_metadata() {
    let node = Node::new(1, None, 0, "test", None);

    if let MetaValue::Map(map) = &node.metadata {
        assert!(map.is_empty());
    } else {
        panic!("Expected default metadata to be an empty Map");
    }
}

/// Tests that a node can be initialized with integer metadata.
#[test]
fn test_node_with_int_metadata() {
    let node = Node::new(1, None, 0, "test", Some(MetaValue::Int(42)));

    if let MetaValue::Int(v) = node.metadata {
        assert_eq!(v, 42);
    } else {
        panic!("Expected MetaValue::Int");
    }
}

/// Tests that a node can be initialized with a map containing multiple values.
#[test]
fn test_node_with_map_metadata() {
    let mut map = HashMap::new();
    map.insert("version".to_string(), MetaValue::Int(1));
    map.insert("name".to_string(), MetaValue::Str("config".to_string()));

    let node = Node::new(1, None, 0, "test", Some(MetaValue::Map(map)));

    if let MetaValue::Map(m) = &node.metadata {
        assert_eq!(m.len(), 2);
        assert!(m.contains_key("version"));
        assert!(m.contains_key("name"));
    } else {
        panic!("Expected MetaValue::Map");
    }
}
