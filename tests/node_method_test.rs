// set_meta and get_meta tests
use synapse_db::node::{MetaValue, Node};

#[test]
fn test_set_meta_on_empty_map() {
    let mut node = Node::new(1, None, 0, "test", None);

    node.set_meta("key", MetaValue::Int(123));

    let value = node.get_meta("key");
    assert!(value.is_some());
    if let Some(MetaValue::Int(v)) = value {
        assert_eq!(*v, 123);
    } else {
        panic!("Expected MetaValue::Int");
    }
}

#[test]
fn test_set_meta_multiple_values() {
    let mut node = Node::new(1, None, 0, "test", None);

    node.set_meta("int_key", MetaValue::Int(42));
    node.set_meta("float_key", MetaValue::Float(1.5));
    node.set_meta("str_key", MetaValue::Str("value".to_string()));

    assert!(matches!(node.get_meta("int_key"), Some(MetaValue::Int(42))));
    if let Some(MetaValue::Float(f)) = node.get_meta("float_key") {
        assert!((f - 1.5).abs() < f64::EPSILON);
    } else {
        panic!("Expected MetaValue::Float");
    }
    if let Some(MetaValue::Str(s)) = node.get_meta("str_key") {
        assert_eq!(s, "value");
    } else {
        panic!("Expected MetaValue::Str");
    }
}

#[test]
fn test_set_meta_overwrites_existing() {
    let mut node = Node::new(1, None, 0, "test", None);

    node.set_meta("key", MetaValue::Int(1));
    node.set_meta("key", MetaValue::Int(2));

    if let Some(MetaValue::Int(v)) = node.get_meta("key") {
        assert_eq!(*v, 2);
    } else {
        panic!("Expected MetaValue::Int");
    }
}

#[test]
fn test_set_meta_converts_primitive_to_map() {
    let mut node = Node::new(1, None, 0, "test", Some(MetaValue::Int(100)));

    node.set_meta("new_key", MetaValue::Str("new_value".to_string()));

    // Original value should be preserved under "value" key
    if let Some(MetaValue::Int(v)) = node.get_meta("value") {
        assert_eq!(*v, 100);
    } else {
        panic!("Expected original value to be preserved");
    }

    // New key should exist
    if let Some(MetaValue::Str(s)) = node.get_meta("new_key") {
        assert_eq!(s, "new_value");
    } else {
        panic!("Expected MetaValue::Str");
    }
}

#[test]
fn test_get_meta_nonexistent_key() {
    let node = Node::new(1, None, 0, "test", None);

    assert!(node.get_meta("nonexistent").is_none());
}

#[test]
fn test_get_meta_on_primitive_metadata() {
    let node = Node::new(1, None, 0, "test", Some(MetaValue::Int(42)));

    // get_meta returns None when metadata is not a Map
    assert!(node.get_meta("any_key").is_none());
}
