//! Unit tests for the `MetaValue` enum.
//!
//! These tests verify that all `MetaValue` variants can be created,
//! stored, and accessed correctly, including nested structures.

use std::collections::HashMap;
use synapse_db::meta_data::MetaValue;

/// Tests that `MetaValue::Int` correctly stores and retrieves an integer.
#[test]
fn test_metavalue_int() {
    let value = MetaValue::Int(42);
    if let MetaValue::Int(v) = value {
        assert_eq!(v, 42);
    } else {
        panic!("Expected MetaValue::Int");
    }
}

/// Tests that `MetaValue::Float` correctly stores and retrieves a float.
#[test]
fn test_metavalue_float() {
    let value = MetaValue::Float(3.5);
    if let MetaValue::Float(v) = value {
        assert!((v - 3.5).abs() < f64::EPSILON);
    } else {
        panic!("Expected MetaValue::Float");
    }
}

/// Tests that `MetaValue::Str` correctly stores and retrieves a string.
#[test]
fn test_metavalue_str() {
    let value = MetaValue::Str("hello".to_string());
    if let MetaValue::Str(v) = value {
        assert_eq!(v, "hello");
    } else {
        panic!("Expected MetaValue::Str");
    }
}

/// Tests that `MetaValue::Map` can store and retrieve key-value pairs.
#[test]
fn test_metavalue_map() {
    let mut map = HashMap::new();
    map.insert("key".to_string(), MetaValue::Int(100));
    let value = MetaValue::Map(map);

    if let MetaValue::Map(m) = value {
        assert!(m.contains_key("key"));
        if let Some(MetaValue::Int(v)) = m.get("key") {
            assert_eq!(*v, 100);
        } else {
            panic!("Expected MetaValue::Int in map");
        }
    } else {
        panic!("Expected MetaValue::Map");
    }
}

/// Tests that `MetaValue::Map` can be nested within another map.
#[test]
fn test_metavalue_nested_map() {
    let mut inner_map = HashMap::new();
    inner_map.insert("nested".to_string(), MetaValue::Str("deep".to_string()));

    let mut outer_map = HashMap::new();
    outer_map.insert("inner".to_string(), MetaValue::Map(inner_map));

    let value = MetaValue::Map(outer_map);

    if let MetaValue::Map(m) = value {
        if let Some(MetaValue::Map(inner)) = m.get("inner") {
            if let Some(MetaValue::Str(s)) = inner.get("nested") {
                assert_eq!(s, "deep");
            } else {
                panic!("Expected MetaValue::Str in nested map");
            }
        } else {
            panic!("Expected MetaValue::Map as inner value");
        }
    } else {
        panic!("Expected MetaValue::Map");
    }
}

/// Tests that `MetaValue::Vec` can store a homogeneous list of integers.
#[test]
fn test_metavalue_vec_with_ints() {
    let vec = MetaValue::Vec(vec![
        MetaValue::Int(1),
        MetaValue::Int(2),
        MetaValue::Int(3),
    ]);

    if let MetaValue::Vec(v) = vec {
        assert_eq!(v.len(), 3);
        if let MetaValue::Int(val) = &v[0] {
            assert_eq!(*val, 1);
        } else {
            panic!("Expected MetaValue::Int");
        }
    } else {
        panic!("Expected MetaValue::Vec");
    }
}

/// Tests that `MetaValue::Vec` can store heterogeneous types.
#[test]
fn test_metavalue_vec_mixed_types() {
    let vec = MetaValue::Vec(vec![
        MetaValue::Int(42),
        MetaValue::Float(3.5),
        MetaValue::Str("hello".to_string()),
    ]);

    if let MetaValue::Vec(v) = vec {
        assert_eq!(v.len(), 3);
        assert!(matches!(&v[0], MetaValue::Int(42)));
        assert!(matches!(&v[2], MetaValue::Str(s) if s == "hello"));
    } else {
        panic!("Expected MetaValue::Vec");
    }
}

/// Tests that `MetaValue::Vec` can be nested within another vector.
#[test]
fn test_metavalue_vec_nested() {
    let inner_vec = MetaValue::Vec(vec![MetaValue::Int(1), MetaValue::Int(2)]);
    let outer_vec = MetaValue::Vec(vec![inner_vec, MetaValue::Int(3)]);

    if let MetaValue::Vec(outer) = outer_vec {
        assert_eq!(outer.len(), 2);
        if let MetaValue::Vec(inner) = &outer[0] {
            assert_eq!(inner.len(), 2);
        } else {
            panic!("Expected nested MetaValue::Vec");
        }
    } else {
        panic!("Expected MetaValue::Vec");
    }
}

/// Tests that an empty `MetaValue::Vec` is handled correctly.
#[test]
fn test_metavalue_vec_empty() {
    let vec = MetaValue::Vec(vec![]);

    if let MetaValue::Vec(v) = vec {
        assert!(v.is_empty());
    } else {
        panic!("Expected MetaValue::Vec");
    }
}
