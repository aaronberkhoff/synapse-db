//! Unit tests for the `EdgeDirection` enum.
//!
//! These tests verify EdgeDirection variants, equality,
//! and serialization behavior.

use synapse_db::edge::EdgeDirection;

/// Tests that EdgeDirection variants can be created and compared.
#[test]
fn test_edge_direction_variants() {
    let atob = EdgeDirection::AtoB;
    let btoa = EdgeDirection::BtoA;
    let bidirectional = EdgeDirection::Bidirectional;

    assert_eq!(atob, EdgeDirection::AtoB);
    assert_eq!(btoa, EdgeDirection::BtoA);
    assert_eq!(bidirectional, EdgeDirection::Bidirectional);
}

/// Tests that different EdgeDirection variants are not equal.
#[test]
fn test_edge_direction_inequality() {
    assert_ne!(EdgeDirection::AtoB, EdgeDirection::BtoA);
    assert_ne!(EdgeDirection::AtoB, EdgeDirection::Bidirectional);
    assert_ne!(EdgeDirection::BtoA, EdgeDirection::Bidirectional);
}

/// Tests that EdgeDirection implements Clone (via Copy).
#[test]
fn test_edge_direction_clone() {
    let original = EdgeDirection::Bidirectional;
    // EdgeDirection is Copy, so clone is implicit
    let cloned: EdgeDirection = Clone::clone(&original);

    assert_eq!(original, cloned);
}

/// Tests that EdgeDirection can be copied.
#[test]
fn test_edge_direction_copy() {
    let original = EdgeDirection::AtoB;
    let copied = original; // Copy, not move

    assert_eq!(original, copied);
    // original is still valid because EdgeDirection is Copy
    assert_eq!(original, EdgeDirection::AtoB);
}

/// Tests that EdgeDirection has Debug formatting.
#[test]
fn test_edge_direction_debug() {
    let atob = EdgeDirection::AtoB;
    let btoa = EdgeDirection::BtoA;
    let bidirectional = EdgeDirection::Bidirectional;

    assert_eq!(format!("{:?}", atob), "AtoB");
    assert_eq!(format!("{:?}", btoa), "BtoA");
    assert_eq!(format!("{:?}", bidirectional), "Bidirectional");
}

/// Tests EdgeDirection serialization to JSON.
#[test]
fn test_edge_direction_serialize() {
    let atob = EdgeDirection::AtoB;
    let btoa = EdgeDirection::BtoA;
    let bidirectional = EdgeDirection::Bidirectional;

    let atob_json = serde_json::to_string(&atob).unwrap();
    let btoa_json = serde_json::to_string(&btoa).unwrap();
    let bidirectional_json = serde_json::to_string(&bidirectional).unwrap();

    assert_eq!(atob_json, "\"AtoB\"");
    assert_eq!(btoa_json, "\"BtoA\"");
    assert_eq!(bidirectional_json, "\"Bidirectional\"");
}

/// Tests EdgeDirection deserialization from JSON.
#[test]
fn test_edge_direction_deserialize() {
    let atob: EdgeDirection = serde_json::from_str("\"AtoB\"").unwrap();
    let btoa: EdgeDirection = serde_json::from_str("\"BtoA\"").unwrap();
    let bidirectional: EdgeDirection = serde_json::from_str("\"Bidirectional\"").unwrap();

    assert_eq!(atob, EdgeDirection::AtoB);
    assert_eq!(btoa, EdgeDirection::BtoA);
    assert_eq!(bidirectional, EdgeDirection::Bidirectional);
}

/// Tests round-trip serialization/deserialization.
#[test]
fn test_edge_direction_roundtrip() {
    let directions = [
        EdgeDirection::AtoB,
        EdgeDirection::BtoA,
        EdgeDirection::Bidirectional,
    ];

    for original in directions {
        let json = serde_json::to_string(&original).unwrap();
        let restored: EdgeDirection = serde_json::from_str(&json).unwrap();
        assert_eq!(original, restored);
    }
}
