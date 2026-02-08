//! Unit tests for the `Graph` struct.
//!
//! These tests verify graph creation, field initialization,
//! and serialization behavior.

use std::collections::HashMap;
use synapse_db::graph::Graph;
use synapse_db::meta_data::MetaValue;

/// Tests that a new graph has an empty set of nodes, edges, and adjacency.
#[test]
fn test_graph_new_empty() {
    let graph = Graph::new("test", None);

    assert_eq!(graph.name, "test");
    assert_eq!(graph.node_count(), 0);
    assert_eq!(graph.edge_count(), 0);
    assert!(graph.nodes.is_empty());
    assert!(graph.edges.is_empty());
    assert!(graph.adjacency.is_empty());
}

/// Tests that a graph created with None metadata defaults to an empty map.
#[test]
fn test_graph_default_metadata() {
    let graph = Graph::new("test", None);

    if let MetaValue::Map(map) = &graph.metadata {
        assert!(map.is_empty());
    } else {
        panic!("Expected default metadata to be an empty Map");
    }
}

/// Tests that a graph can be initialized with custom metadata.
#[test]
fn test_graph_with_metadata() {
    let mut map = HashMap::new();
    map.insert("version".to_string(), MetaValue::Int(1));
    let graph = Graph::new("test", Some(MetaValue::Map(map)));

    if let MetaValue::Map(m) = &graph.metadata {
        assert_eq!(m.len(), 1);
        assert!(m.contains_key("version"));
    } else {
        panic!("Expected MetaValue::Map");
    }
}

/// Tests that node IDs are auto-incremented starting from 1.
#[test]
fn test_graph_node_id_auto_increment() {
    let mut graph = Graph::new("test", None);

    let id1 = graph.add_node("first", None);
    let id2 = graph.add_node("second", None);
    let id3 = graph.add_node("third", None);

    assert_eq!(id1, 1);
    assert_eq!(id2, 2);
    assert_eq!(id3, 3);
}

/// Tests that edge IDs are auto-incremented starting from 1.
#[test]
fn test_graph_edge_id_auto_increment() {
    let mut graph = Graph::new("test", None);
    let a = graph.add_node("a", None);
    let b = graph.add_node("b", None);
    let c = graph.add_node("c", None);

    let e1 = graph.add_edge(a, b, false, "e1", None).unwrap();
    let e2 = graph.add_edge(b, c, false, "e2", None).unwrap();

    assert_eq!(e1, 1);
    assert_eq!(e2, 2);
}

/// Tests that a graph can be serialized to JSON and deserialized back.
#[test]
fn test_graph_serialization_roundtrip() {
    let mut graph = Graph::new("roundtrip", None);
    let a = graph.add_node("alice", None);
    let b = graph.add_node("bob", None);
    graph.add_edge(a, b, false, "follows", None).unwrap();

    let json = serde_json::to_string(&graph).unwrap();
    let restored: Graph = serde_json::from_str(&json).unwrap();

    assert_eq!(restored.name, "roundtrip");
    assert_eq!(restored.node_count(), 2);
    assert_eq!(restored.edge_count(), 1);
}

/// Tests that cloning a graph produces an independent copy.
#[test]
fn test_graph_clone_independence() {
    let mut graph = Graph::new("original", None);
    graph.add_node("node", None);

    let mut cloned = graph.clone();
    cloned.add_node("extra", None);

    assert_eq!(graph.node_count(), 1);
    assert_eq!(cloned.node_count(), 2);
}
