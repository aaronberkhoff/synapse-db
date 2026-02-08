//! Unit tests for `Edge` methods.
//!
//! These tests verify the connects, connects_pair, allows_traversal,
//! set_meta, and get_meta methods.

use synapse_db::edge::{Edge, EdgeDirection};
use synapse_db::meta_data::{HasMetadata, MetaValue};

// ============================================================================
// connects() tests
// ============================================================================

#[test]
fn test_connects_node_a() {
    let edge = Edge::new(1, 2, 5, false, "link", None);

    assert!(edge.connects(2));
}

#[test]
fn test_connects_node_b() {
    let edge = Edge::new(1, 2, 5, false, "link", None);

    assert!(edge.connects(5));
}

#[test]
fn test_connects_unrelated_node() {
    let edge = Edge::new(1, 2, 5, false, "link", None);

    assert!(!edge.connects(3));
    assert!(!edge.connects(0));
    assert!(!edge.connects(-1));
}

// ============================================================================
// connects_pair() tests
// ============================================================================

#[test]
fn test_connects_pair_ascending_order() {
    let edge = Edge::new(1, 2, 5, false, "link", None);

    assert!(edge.connects_pair(2, 5));
}

#[test]
fn test_connects_pair_descending_order() {
    let edge = Edge::new(1, 2, 5, false, "link", None);

    // Order shouldn't matter
    assert!(edge.connects_pair(5, 2));
}

#[test]
fn test_connects_pair_wrong_nodes() {
    let edge = Edge::new(1, 2, 5, false, "link", None);

    assert!(!edge.connects_pair(2, 3));
    assert!(!edge.connects_pair(1, 5));
    assert!(!edge.connects_pair(3, 4));
}

#[test]
fn test_connects_pair_partial_match() {
    let edge = Edge::new(1, 2, 5, false, "link", None);

    // One node matches, but not both
    assert!(!edge.connects_pair(2, 6));
    assert!(!edge.connects_pair(1, 5));
}

// ============================================================================
// allows_traversal() tests - Directed edges
// ============================================================================

#[test]
fn test_allows_traversal_atob_forward() {
    // Edge created as 2 -> 5, so direction is AtoB
    let edge = Edge::new(1, 2, 5, false, "follows", None);

    assert_eq!(edge.direction, EdgeDirection::AtoB);
    assert!(edge.allows_traversal(2, 5)); // forward allowed
}

#[test]
fn test_allows_traversal_atob_backward() {
    let edge = Edge::new(1, 2, 5, false, "follows", None);

    assert!(!edge.allows_traversal(5, 2)); // backward blocked
}

#[test]
fn test_allows_traversal_btoa_forward() {
    // Edge created as 5 -> 2, so direction is BtoA (since 5 > 2)
    let edge = Edge::new(1, 5, 2, false, "follows", None);

    assert_eq!(edge.direction, EdgeDirection::BtoA);
    assert!(edge.allows_traversal(5, 2)); // original direction allowed
}

#[test]
fn test_allows_traversal_btoa_backward() {
    let edge = Edge::new(1, 5, 2, false, "follows", None);

    assert!(!edge.allows_traversal(2, 5)); // reverse direction blocked
}

// ============================================================================
// allows_traversal() tests - Bidirectional edges
// ============================================================================

#[test]
fn test_allows_traversal_bidirectional_forward() {
    let edge = Edge::new(1, 2, 5, true, "friends", None);

    assert_eq!(edge.direction, EdgeDirection::Bidirectional);
    assert!(edge.allows_traversal(2, 5));
}

#[test]
fn test_allows_traversal_bidirectional_backward() {
    let edge = Edge::new(1, 2, 5, true, "friends", None);

    assert!(edge.allows_traversal(5, 2));
}

#[test]
fn test_allows_traversal_wrong_nodes() {
    let edge = Edge::new(1, 2, 5, true, "friends", None);

    // Nodes that aren't part of this edge
    assert!(!edge.allows_traversal(2, 3));
    assert!(!edge.allows_traversal(1, 5));
    assert!(!edge.allows_traversal(3, 4));
}

// ============================================================================
// set_meta() and get_meta() tests
// ============================================================================

#[test]
fn test_set_meta_on_empty_map() {
    let mut edge = Edge::new(1, 2, 5, false, "link", None);

    edge.set_meta("weight", MetaValue::Float(1.5));

    let value = edge.get_meta("weight");
    assert!(value.is_some());
    if let Some(MetaValue::Float(v)) = value {
        assert!((v - 1.5).abs() < f64::EPSILON);
    } else {
        panic!("Expected MetaValue::Float");
    }
}

#[test]
fn test_set_meta_multiple_values() {
    let mut edge = Edge::new(1, 2, 5, false, "link", None);

    edge.set_meta("weight", MetaValue::Float(2.0));
    edge.set_meta("label", MetaValue::Str("important".to_string()));
    edge.set_meta("count", MetaValue::Int(10));

    if let Some(MetaValue::Float(f)) = edge.get_meta("weight") {
        assert!((f - 2.0).abs() < f64::EPSILON);
    } else {
        panic!("Expected MetaValue::Float");
    }

    if let Some(MetaValue::Str(s)) = edge.get_meta("label") {
        assert_eq!(s, "important");
    } else {
        panic!("Expected MetaValue::Str");
    }

    assert!(matches!(edge.get_meta("count"), Some(MetaValue::Int(10))));
}

#[test]
fn test_set_meta_overwrites_existing() {
    let mut edge = Edge::new(1, 2, 5, false, "link", None);

    edge.set_meta("weight", MetaValue::Float(1.0));
    edge.set_meta("weight", MetaValue::Float(2.0));

    if let Some(MetaValue::Float(v)) = edge.get_meta("weight") {
        assert!((v - 2.0).abs() < f64::EPSILON);
    } else {
        panic!("Expected MetaValue::Float");
    }
}

#[test]
fn test_set_meta_converts_primitive_to_map() {
    let mut edge = Edge::new(1, 2, 5, false, "link", Some(MetaValue::Float(1.5)));

    edge.set_meta("label", MetaValue::Str("converted".to_string()));

    // Original value should be preserved under "value" key
    if let Some(MetaValue::Float(v)) = edge.get_meta("value") {
        assert!((v - 1.5).abs() < f64::EPSILON);
    } else {
        panic!("Expected original value to be preserved");
    }

    // New key should exist
    if let Some(MetaValue::Str(s)) = edge.get_meta("label") {
        assert_eq!(s, "converted");
    } else {
        panic!("Expected MetaValue::Str");
    }
}

#[test]
fn test_get_meta_nonexistent_key() {
    let edge = Edge::new(1, 2, 5, false, "link", None);

    assert!(edge.get_meta("nonexistent").is_none());
}

#[test]
fn test_get_meta_on_primitive_metadata() {
    let edge = Edge::new(1, 2, 5, false, "link", Some(MetaValue::Int(42)));

    // get_meta returns None when metadata is not a Map
    assert!(edge.get_meta("any_key").is_none());
}
