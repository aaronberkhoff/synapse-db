//! Unit tests for `Graph` methods.
//!
//! These tests verify node/edge operations, query methods,
//! adjacency index maintenance, and metadata handling.

use synapse_db::edge::EdgeDirection;
use synapse_db::graph::Graph;
use synapse_db::meta_data::{HasMetadata, MetaValue};

// ============================================================================
// Node operations
// ============================================================================

/// Tests that add_node returns incrementing IDs and stores nodes.
#[test]
fn test_add_node_returns_incrementing_ids() {
    let mut graph = Graph::new("test", None);

    let id1 = graph.add_node("a", None);
    let id2 = graph.add_node("b", None);

    assert_eq!(id1, 1);
    assert_eq!(id2, 2);
    assert_eq!(graph.node_count(), 2);
}

/// Tests that add_node initializes an empty adjacency set.
#[test]
fn test_add_node_initializes_adjacency() {
    let mut graph = Graph::new("test", None);

    let id = graph.add_node("a", None);

    let edges = graph.edges_of(id).unwrap();
    assert!(edges.is_empty());
}

/// Tests that remove_node returns the removed node.
#[test]
fn test_remove_node_success() {
    let mut graph = Graph::new("test", None);
    let id = graph.add_node("a", None);

    let removed = graph.remove_node(id).unwrap();

    assert_eq!(removed.id, id);
    assert_eq!(removed.name, "a");
    assert_eq!(graph.node_count(), 0);
    assert!(!graph.has_node(id));
}

/// Tests that remove_node returns an error for nonexistent node.
#[test]
fn test_remove_node_not_found() {
    let mut graph = Graph::new("test", None);

    let result = graph.remove_node(999);

    assert!(result.is_err());
}

/// Tests that remove_node also removes all connected edges.
#[test]
fn test_remove_node_removes_connected_edges() {
    let mut graph = Graph::new("test", None);
    let a = graph.add_node("a", None);
    let b = graph.add_node("b", None);
    let c = graph.add_node("c", None);
    let e1 = graph.add_edge(a, b, false, "ab", None).unwrap();
    let e2 = graph.add_edge(a, c, false, "ac", None).unwrap();

    graph.remove_node(a).unwrap();

    assert_eq!(graph.edge_count(), 0);
    assert!(!graph.has_edge(e1));
    assert!(!graph.has_edge(e2));
}

/// Tests that removing a node cleans up adjacency of its neighbors.
#[test]
fn test_remove_node_cleans_adjacency_of_neighbors() {
    let mut graph = Graph::new("test", None);
    let a = graph.add_node("a", None);
    let b = graph.add_node("b", None);
    graph.add_edge(a, b, false, "ab", None).unwrap();

    graph.remove_node(a).unwrap();

    // b should have no edges in its adjacency
    let edges = graph.edges_of(b).unwrap();
    assert!(edges.is_empty());
}

/// Tests get_node returns the correct node.
#[test]
fn test_get_node_success() {
    let mut graph = Graph::new("test", None);
    let id = graph.add_node("alice", None);

    let node = graph.get_node(id).unwrap();

    assert_eq!(node.id, id);
    assert_eq!(node.name, "alice");
}

/// Tests get_node returns an error for nonexistent node.
#[test]
fn test_get_node_not_found() {
    let graph = Graph::new("test", None);

    let result = graph.get_node(999);

    assert!(result.is_err());
}

/// Tests get_node_mut allows modifying a node.
#[test]
fn test_get_node_mut_modifies_node() {
    let mut graph = Graph::new("test", None);
    let id = graph.add_node("original", None);

    let node = graph.get_node_mut(id).unwrap();
    node.name = "modified".to_string();

    assert_eq!(graph.get_node(id).unwrap().name, "modified");
}

/// Tests has_node returns correct results.
#[test]
fn test_has_node() {
    let mut graph = Graph::new("test", None);
    let id = graph.add_node("a", None);

    assert!(graph.has_node(id));
    assert!(!graph.has_node(999));
}

/// Tests node_count reflects the current state.
#[test]
fn test_node_count() {
    let mut graph = Graph::new("test", None);

    assert_eq!(graph.node_count(), 0);
    let id = graph.add_node("a", None);
    assert_eq!(graph.node_count(), 1);
    graph.remove_node(id).unwrap();
    assert_eq!(graph.node_count(), 0);
}

// ============================================================================
// Edge operations
// ============================================================================

/// Tests that add_edge creates an edge between existing nodes.
#[test]
fn test_add_edge_success() {
    let mut graph = Graph::new("test", None);
    let a = graph.add_node("a", None);
    let b = graph.add_node("b", None);

    let edge_id = graph.add_edge(a, b, false, "link", None).unwrap();

    assert_eq!(graph.edge_count(), 1);
    let edge = graph.get_edge(edge_id).unwrap();
    assert_eq!(edge.name, "link");
}

/// Tests add_edge returns incrementing IDs.
#[test]
fn test_add_edge_returns_incrementing_ids() {
    let mut graph = Graph::new("test", None);
    let a = graph.add_node("a", None);
    let b = graph.add_node("b", None);
    let c = graph.add_node("c", None);

    let e1 = graph.add_edge(a, b, false, "ab", None).unwrap();
    let e2 = graph.add_edge(b, c, false, "bc", None).unwrap();

    assert_eq!(e1, 1);
    assert_eq!(e2, 2);
}

/// Tests add_edge fails when from_node doesn't exist.
#[test]
fn test_add_edge_missing_from_node() {
    let mut graph = Graph::new("test", None);
    let b = graph.add_node("b", None);

    let result = graph.add_edge(999, b, false, "link", None);

    assert!(result.is_err());
}

/// Tests add_edge fails when to_node doesn't exist.
#[test]
fn test_add_edge_missing_to_node() {
    let mut graph = Graph::new("test", None);
    let a = graph.add_node("a", None);

    let result = graph.add_edge(a, 999, false, "link", None);

    assert!(result.is_err());
}

/// Tests that add_edge updates adjacency for both endpoints.
#[test]
fn test_add_edge_updates_adjacency_both_nodes() {
    let mut graph = Graph::new("test", None);
    let a = graph.add_node("a", None);
    let b = graph.add_node("b", None);

    let edge_id = graph.add_edge(a, b, false, "link", None).unwrap();

    assert!(graph.edges_of(a).unwrap().contains(&edge_id));
    assert!(graph.edges_of(b).unwrap().contains(&edge_id));
}

/// Tests that a self-loop edge works correctly.
#[test]
fn test_add_edge_self_loop() {
    let mut graph = Graph::new("test", None);
    let a = graph.add_node("a", None);

    let edge_id = graph.add_edge(a, a, false, "self", None).unwrap();

    assert_eq!(graph.edge_count(), 1);
    assert!(graph.edges_of(a).unwrap().contains(&edge_id));
}

/// Tests remove_edge returns the removed edge.
#[test]
fn test_remove_edge_success() {
    let mut graph = Graph::new("test", None);
    let a = graph.add_node("a", None);
    let b = graph.add_node("b", None);
    let edge_id = graph.add_edge(a, b, false, "link", None).unwrap();

    let removed = graph.remove_edge(edge_id).unwrap();

    assert_eq!(removed.name, "link");
    assert_eq!(graph.edge_count(), 0);
}

/// Tests remove_edge returns an error for nonexistent edge.
#[test]
fn test_remove_edge_not_found() {
    let mut graph = Graph::new("test", None);

    let result = graph.remove_edge(999);

    assert!(result.is_err());
}

/// Tests that remove_edge cleans up adjacency for both endpoints.
#[test]
fn test_remove_edge_cleans_adjacency() {
    let mut graph = Graph::new("test", None);
    let a = graph.add_node("a", None);
    let b = graph.add_node("b", None);
    let edge_id = graph.add_edge(a, b, false, "link", None).unwrap();

    graph.remove_edge(edge_id).unwrap();

    assert!(graph.edges_of(a).unwrap().is_empty());
    assert!(graph.edges_of(b).unwrap().is_empty());
}

/// Tests get_edge returns the correct edge.
#[test]
fn test_get_edge_success() {
    let mut graph = Graph::new("test", None);
    let a = graph.add_node("a", None);
    let b = graph.add_node("b", None);
    let edge_id = graph.add_edge(a, b, false, "link", None).unwrap();

    let edge = graph.get_edge(edge_id).unwrap();

    assert_eq!(edge.id, edge_id);
    assert_eq!(edge.name, "link");
}

/// Tests get_edge returns an error for nonexistent edge.
#[test]
fn test_get_edge_not_found() {
    let graph = Graph::new("test", None);

    let result = graph.get_edge(999);

    assert!(result.is_err());
}

/// Tests has_edge returns correct results.
#[test]
fn test_has_edge() {
    let mut graph = Graph::new("test", None);
    let a = graph.add_node("a", None);
    let b = graph.add_node("b", None);
    let edge_id = graph.add_edge(a, b, false, "link", None).unwrap();

    assert!(graph.has_edge(edge_id));
    assert!(!graph.has_edge(999));
}

/// Tests edge_count reflects the current state.
#[test]
fn test_edge_count() {
    let mut graph = Graph::new("test", None);
    let a = graph.add_node("a", None);
    let b = graph.add_node("b", None);

    assert_eq!(graph.edge_count(), 0);
    let eid = graph.add_edge(a, b, false, "link", None).unwrap();
    assert_eq!(graph.edge_count(), 1);
    graph.remove_edge(eid).unwrap();
    assert_eq!(graph.edge_count(), 0);
}

// ============================================================================
// Query operations
// ============================================================================

/// Tests edges_of returns all edge IDs for a node.
#[test]
fn test_edges_of_node() {
    let mut graph = Graph::new("test", None);
    let a = graph.add_node("a", None);
    let b = graph.add_node("b", None);
    let c = graph.add_node("c", None);
    let e1 = graph.add_edge(a, b, false, "ab", None).unwrap();
    let e2 = graph.add_edge(a, c, false, "ac", None).unwrap();

    let edges = graph.edges_of(a).unwrap();

    assert_eq!(edges.len(), 2);
    assert!(edges.contains(&e1));
    assert!(edges.contains(&e2));
}

/// Tests edges_of returns an error for nonexistent node.
#[test]
fn test_edges_of_nonexistent_node() {
    let graph = Graph::new("test", None);

    let result = graph.edges_of(999);

    assert!(result.is_err());
}

/// Tests neighbors returns all connected nodes for a bidirectional edge.
#[test]
fn test_neighbors_bidirectional_edge() {
    let mut graph = Graph::new("test", None);
    let a = graph.add_node("a", None);
    let b = graph.add_node("b", None);
    graph.add_edge(a, b, true, "friends", None).unwrap();

    let neighbors_a = graph.neighbors(a).unwrap();
    let neighbors_b = graph.neighbors(b).unwrap();

    assert_eq!(neighbors_a.len(), 1);
    assert!(neighbors_a.contains(&b));
    assert_eq!(neighbors_b.len(), 1);
    assert!(neighbors_b.contains(&a));
}

/// Tests neighbors returns connected nodes regardless of edge direction.
#[test]
fn test_neighbors_directed_edge() {
    let mut graph = Graph::new("test", None);
    let a = graph.add_node("a", None);
    let b = graph.add_node("b", None);
    graph.add_edge(a, b, false, "follows", None).unwrap();

    // neighbors() ignores direction
    let neighbors_a = graph.neighbors(a).unwrap();
    let neighbors_b = graph.neighbors(b).unwrap();

    assert_eq!(neighbors_a.len(), 1);
    assert!(neighbors_a.contains(&b));
    assert_eq!(neighbors_b.len(), 1);
    assert!(neighbors_b.contains(&a));
}

/// Tests neighbors with multiple edges deduplicates node IDs.
#[test]
fn test_neighbors_multiple_edges_deduplicates() {
    let mut graph = Graph::new("test", None);
    let a = graph.add_node("a", None);
    let b = graph.add_node("b", None);
    // Two edges between the same pair
    graph.add_edge(a, b, false, "follows", None).unwrap();
    graph.add_edge(a, b, true, "friends", None).unwrap();

    let neighbors_a = graph.neighbors(a).unwrap();

    assert_eq!(neighbors_a.len(), 1); // deduplicated
    assert!(neighbors_a.contains(&b));
}

/// Tests neighbors returns empty vec for isolated node.
#[test]
fn test_neighbors_isolated_node() {
    let mut graph = Graph::new("test", None);
    let a = graph.add_node("a", None);

    let neighbors = graph.neighbors(a).unwrap();

    assert!(neighbors.is_empty());
}

/// Tests directed_neighbors respects edge direction.
#[test]
fn test_directed_neighbors_respects_direction() {
    let mut graph = Graph::new("test", None);
    let a = graph.add_node("a", None);
    let b = graph.add_node("b", None);
    // Directed edge: a -> b
    graph.add_edge(a, b, false, "follows", None).unwrap();

    let from_a = graph.directed_neighbors(a).unwrap();
    let from_b = graph.directed_neighbors(b).unwrap();

    assert_eq!(from_a.len(), 1);
    assert!(from_a.contains(&b));
    assert!(from_b.is_empty()); // can't traverse b -> a
}

/// Tests directed_neighbors includes bidirectional edges.
#[test]
fn test_directed_neighbors_bidirectional() {
    let mut graph = Graph::new("test", None);
    let a = graph.add_node("a", None);
    let b = graph.add_node("b", None);
    graph.add_edge(a, b, true, "friends", None).unwrap();

    let from_a = graph.directed_neighbors(a).unwrap();
    let from_b = graph.directed_neighbors(b).unwrap();

    assert_eq!(from_a.len(), 1);
    assert!(from_a.contains(&b));
    assert_eq!(from_b.len(), 1);
    assert!(from_b.contains(&a));
}

/// Tests directed_neighbors with mixed directed and bidirectional edges.
#[test]
fn test_directed_neighbors_mixed() {
    let mut graph = Graph::new("test", None);
    let a = graph.add_node("a", None);
    let b = graph.add_node("b", None);
    let c = graph.add_node("c", None);

    graph.add_edge(a, b, false, "follows", None).unwrap(); // a -> b
    graph.add_edge(c, a, false, "follows", None).unwrap(); // c -> a

    let from_a = graph.directed_neighbors(a).unwrap();

    // a can reach b (a->b), but not c (edge is c->a, not a->c)
    assert_eq!(from_a.len(), 1);
    assert!(from_a.contains(&b));
}

/// Tests has_edge_between returns true when edges exist.
#[test]
fn test_has_edge_between_true() {
    let mut graph = Graph::new("test", None);
    let a = graph.add_node("a", None);
    let b = graph.add_node("b", None);
    graph.add_edge(a, b, false, "link", None).unwrap();

    assert!(graph.has_edge_between(a, b).unwrap());
    assert!(graph.has_edge_between(b, a).unwrap()); // order doesn't matter
}

/// Tests has_edge_between returns false when no edge exists.
#[test]
fn test_has_edge_between_false() {
    let mut graph = Graph::new("test", None);
    let a = graph.add_node("a", None);
    let b = graph.add_node("b", None);

    assert!(!graph.has_edge_between(a, b).unwrap());
}

/// Tests has_edge_between returns error for nonexistent node.
#[test]
fn test_has_edge_between_nonexistent_node() {
    let mut graph = Graph::new("test", None);
    let a = graph.add_node("a", None);

    let result = graph.has_edge_between(a, 999);

    assert!(result.is_err());
}

/// Tests edges_between returns all edges between two nodes.
#[test]
fn test_edges_between() {
    let mut graph = Graph::new("test", None);
    let a = graph.add_node("a", None);
    let b = graph.add_node("b", None);
    graph.add_edge(a, b, false, "follows", None).unwrap();
    graph.add_edge(a, b, true, "friends", None).unwrap();

    let edges = graph.edges_between(a, b).unwrap();

    assert_eq!(edges.len(), 2);
}

/// Tests that edge direction is correctly set through the graph.
#[test]
fn test_edge_direction_through_graph() {
    let mut graph = Graph::new("test", None);
    let a = graph.add_node("a", None);
    let b = graph.add_node("b", None);

    let e1 = graph.add_edge(a, b, false, "directed", None).unwrap();
    let e2 = graph.add_edge(a, b, true, "bidir", None).unwrap();

    let edge1 = graph.get_edge(e1).unwrap();
    let edge2 = graph.get_edge(e2).unwrap();

    assert_eq!(edge1.direction, EdgeDirection::AtoB);
    assert_eq!(edge2.direction, EdgeDirection::Bidirectional);
}

/// Tests degree returns the correct number of edges for a node.
#[test]
fn test_degree() {
    let mut graph = Graph::new("test", None);
    let a = graph.add_node("a", None);
    let b = graph.add_node("b", None);
    let c = graph.add_node("c", None);
    graph.add_edge(a, b, false, "ab", None).unwrap();
    graph.add_edge(a, c, false, "ac", None).unwrap();

    assert_eq!(graph.degree(a).unwrap(), 2);
    assert_eq!(graph.degree(b).unwrap(), 1);
    assert_eq!(graph.degree(c).unwrap(), 1);
}

/// Tests degree returns error for nonexistent node.
#[test]
fn test_degree_nonexistent_node() {
    let graph = Graph::new("test", None);

    let result = graph.degree(999);

    assert!(result.is_err());
}

// ============================================================================
// Metadata operations
// ============================================================================

/// Tests graph-level set_meta and get_meta.
#[test]
fn test_graph_set_and_get_meta() {
    let mut graph = Graph::new("test", None);

    graph.set_meta("version", MetaValue::Int(1));

    let value = graph.get_meta("version");
    assert!(matches!(value, Some(MetaValue::Int(1))));
}

/// Tests graph metadata with multiple values.
#[test]
fn test_graph_metadata_multiple_values() {
    let mut graph = Graph::new("test", None);

    graph.set_meta("version", MetaValue::Int(1));
    graph.set_meta("author", MetaValue::Str("test".to_string()));

    assert!(matches!(graph.get_meta("version"), Some(MetaValue::Int(1))));
    assert!(matches!(
        graph.get_meta("author"),
        Some(MetaValue::Str(s)) if s == "test"
    ));
}

// ============================================================================
// Utility operations
// ============================================================================

/// Tests rebuild_adjacency reconstructs the index correctly.
#[test]
fn test_rebuild_adjacency() {
    let mut graph = Graph::new("test", None);
    let a = graph.add_node("a", None);
    let b = graph.add_node("b", None);
    let c = graph.add_node("c", None);
    let e1 = graph.add_edge(a, b, false, "ab", None).unwrap();
    let e2 = graph.add_edge(b, c, false, "bc", None).unwrap();

    // Clear adjacency manually
    graph.adjacency.clear();
    assert!(graph.adjacency.is_empty());

    // Rebuild
    graph.rebuild_adjacency();

    // Verify
    assert!(graph.adjacency.get(&a).unwrap().contains(&e1));
    assert!(graph.adjacency.get(&b).unwrap().contains(&e1));
    assert!(graph.adjacency.get(&b).unwrap().contains(&e2));
    assert!(graph.adjacency.get(&c).unwrap().contains(&e2));
    assert_eq!(graph.adjacency.get(&a).unwrap().len(), 1);
    assert_eq!(graph.adjacency.get(&b).unwrap().len(), 2);
    assert_eq!(graph.adjacency.get(&c).unwrap().len(), 1);
}
