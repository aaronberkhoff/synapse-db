//! Unit tests for the `Edge` struct.
//!
//! These tests verify edge creation, canonical ordering,
//! and direction handling functionality.

use std::collections::HashMap;
use synapse_db::edge::{Edge, EdgeDirection};
use synapse_db::meta_data::MetaValue;

/// Tests that an edge is created with canonical ordering (node_a < node_b).
#[test]
fn test_edge_canonical_ordering_ascending() {
    let edge = Edge::new(1, 2, 5, false, "link", None);

    assert_eq!(edge.id, 1);
    assert_eq!(edge.node_a, 2); // smaller ID
    assert_eq!(edge.node_b, 5); // larger ID
    assert_eq!(edge.name, "link");
    assert_eq!(edge.direction, EdgeDirection::AtoB);
}

/// Tests that an edge with reversed node IDs is still stored canonically.
#[test]
fn test_edge_canonical_ordering_descending() {
    let edge = Edge::new(1, 5, 2, false, "link", None);

    assert_eq!(edge.node_a, 2); // smaller ID
    assert_eq!(edge.node_b, 5); // larger ID
    assert_eq!(edge.direction, EdgeDirection::BtoA); // direction reflects original order
}

/// Tests that equal node IDs are handled correctly.
#[test]
fn test_edge_equal_node_ids() {
    let edge = Edge::new(1, 3, 3, false, "self-loop", None);

    assert_eq!(edge.node_a, 3);
    assert_eq!(edge.node_b, 3);
    assert_eq!(edge.direction, EdgeDirection::AtoB);
}

/// Tests bidirectional edge creation with ascending IDs.
#[test]
fn test_edge_bidirectional_ascending() {
    let edge = Edge::new(1, 2, 5, true, "friends", None);

    assert_eq!(edge.node_a, 2);
    assert_eq!(edge.node_b, 5);
    assert_eq!(edge.direction, EdgeDirection::Bidirectional);
}

/// Tests bidirectional edge creation with descending IDs.
#[test]
fn test_edge_bidirectional_descending() {
    let edge = Edge::new(1, 5, 2, true, "friends", None);

    assert_eq!(edge.node_a, 2);
    assert_eq!(edge.node_b, 5);
    assert_eq!(edge.direction, EdgeDirection::Bidirectional);
}

/// Tests that an edge created with None metadata defaults to an empty map.
#[test]
fn test_edge_default_metadata() {
    let edge = Edge::new(1, 2, 5, false, "link", None);

    if let MetaValue::Map(map) = &edge.metadata {
        assert!(map.is_empty());
    } else {
        panic!("Expected default metadata to be an empty Map");
    }
}

/// Tests that an edge can be initialized with integer metadata.
#[test]
fn test_edge_with_int_metadata() {
    let edge = Edge::new(1, 2, 5, false, "link", Some(MetaValue::Int(42)));

    if let MetaValue::Int(v) = edge.metadata {
        assert_eq!(v, 42);
    } else {
        panic!("Expected MetaValue::Int");
    }
}

/// Tests that an edge can be initialized with float metadata (for weights).
#[test]
fn test_edge_with_float_metadata() {
    let edge = Edge::new(1, 2, 5, false, "link", Some(MetaValue::Float(1.5)));

    if let MetaValue::Float(v) = edge.metadata {
        assert!((v - 1.5).abs() < f64::EPSILON);
    } else {
        panic!("Expected MetaValue::Float");
    }
}

/// Tests that an edge can be initialized with a map containing multiple values.
#[test]
fn test_edge_with_map_metadata() {
    let mut map = HashMap::new();
    map.insert("weight".to_string(), MetaValue::Float(2.5));
    map.insert("label".to_string(), MetaValue::Str("important".to_string()));

    let edge = Edge::new(1, 2, 5, false, "link", Some(MetaValue::Map(map)));

    if let MetaValue::Map(m) = &edge.metadata {
        assert_eq!(m.len(), 2);
        assert!(m.contains_key("weight"));
        assert!(m.contains_key("label"));
    } else {
        panic!("Expected MetaValue::Map");
    }
}

/// Tests edge with negative node IDs (canonical ordering still applies).
#[test]
fn test_edge_negative_node_ids() {
    let edge = Edge::new(1, -5, -2, false, "link", None);

    assert_eq!(edge.node_a, -5); // smaller (more negative)
    assert_eq!(edge.node_b, -2); // larger (less negative)
    assert_eq!(edge.direction, EdgeDirection::AtoB);
}

/// Tests edge with mixed positive and negative node IDs.
#[test]
fn test_edge_mixed_sign_node_ids() {
    let edge = Edge::new(1, 5, -3, false, "link", None);

    assert_eq!(edge.node_a, -3); // smaller (negative)
    assert_eq!(edge.node_b, 5); // larger (positive)
    assert_eq!(edge.direction, EdgeDirection::BtoA);
}
