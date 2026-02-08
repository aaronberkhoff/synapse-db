//! Graph struct for managing collections of nodes and edges.
//!
//! The `Graph` provides an adjacency-indexed container for `Node` and `Edge`
//! types, supporting fast neighbor lookups, directed and bidirectional edges,
//! and flexible metadata storage.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use thiserror::Error;

use crate::edge::Edge;
use crate::meta_data::{HasMetadata, MetaValue};
use crate::node::Node;

/// Errors that can occur during graph operations.
#[derive(Debug, Clone, Error)]
pub enum GraphError {
    /// The specified node was not found in the graph.
    #[error("Node with id {0} not found")]
    NodeNotFound(i64),

    /// The specified edge was not found in the graph.
    #[error("Edge with id {0} not found")]
    EdgeNotFound(i64),

    /// A node referenced by an edge does not exist in the graph.
    #[error("Cannot create edge: node {0} does not exist")]
    EdgeNodeMissing(i64),
}

/// A convenience type alias for graph operations.
pub type GraphResult<T> = Result<T, GraphError>;

/// A graph data structure containing nodes and edges with an adjacency index.
///
/// The adjacency index maps each node ID to the set of edge IDs that
/// connect to it, enabling fast neighbor lookups.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Graph {
    pub nodes: HashMap<i64, Node>,
    pub edges: HashMap<i64, Edge>,
    /// Maps each node ID to the set of edge IDs that touch it.
    pub adjacency: HashMap<i64, HashSet<i64>>,
    pub name: String,
    pub metadata: MetaValue,
    next_node_id: i64,
    next_edge_id: i64,
}

impl Graph {
    /// Creates a new empty graph.
    ///
    /// # Arguments
    /// * `name` - The name of the graph
    /// * `metadata` - Optional metadata for the graph
    pub fn new(name: &str, metadata: Option<MetaValue>) -> Self {
        Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
            adjacency: HashMap::new(),
            name: name.to_string(),
            metadata: metadata.unwrap_or(MetaValue::Map(HashMap::new())),
            next_node_id: 1,
            next_edge_id: 1,
        }
    }

    // ========================================================================
    // Node operations
    // ========================================================================

    /// Adds a new node to the graph with an auto-assigned ID.
    ///
    /// Returns the assigned node ID.
    pub fn add_node(&mut self, name: &str, metadata: Option<MetaValue>) -> i64 {
        let id = self.next_node_id;
        let node = Node::new(id, None, 0, name, metadata);
        self.nodes.insert(id, node);
        self.adjacency.insert(id, HashSet::new());
        self.next_node_id += 1;
        id
    }

    /// Removes a node and all its connected edges from the graph.
    ///
    /// Returns the removed Node, or an error if the node does not exist.
    pub fn remove_node(&mut self, node_id: i64) -> GraphResult<Node> {
        if !self.nodes.contains_key(&node_id) {
            return Err(GraphError::NodeNotFound(node_id));
        }

        // Clone the edge set to avoid borrow conflict
        let edge_ids: Vec<i64> = self
            .adjacency
            .get(&node_id)
            .map(|set| set.iter().copied().collect())
            .unwrap_or_default();

        for edge_id in edge_ids {
            if let Some(edge) = self.edges.remove(&edge_id) {
                // Clean up the other node's adjacency entry
                let other_node = if edge.node_a == node_id {
                    edge.node_b
                } else {
                    edge.node_a
                };
                if let Some(adj) = self.adjacency.get_mut(&other_node) {
                    adj.remove(&edge_id);
                }
            }
        }

        self.adjacency.remove(&node_id);
        Ok(self.nodes.remove(&node_id).unwrap())
    }

    /// Returns a reference to a node by ID, or an error if not found.
    pub fn get_node(&self, node_id: i64) -> GraphResult<&Node> {
        self.nodes
            .get(&node_id)
            .ok_or(GraphError::NodeNotFound(node_id))
    }

    /// Returns a mutable reference to a node by ID, or an error if not found.
    pub fn get_node_mut(&mut self, node_id: i64) -> GraphResult<&mut Node> {
        self.nodes
            .get_mut(&node_id)
            .ok_or(GraphError::NodeNotFound(node_id))
    }

    /// Returns true if a node with the given ID exists.
    pub fn has_node(&self, node_id: i64) -> bool {
        self.nodes.contains_key(&node_id)
    }

    /// Returns the number of nodes in the graph.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    // ========================================================================
    // Edge operations
    // ========================================================================

    /// Adds a new edge between two nodes with an auto-assigned ID.
    ///
    /// Both nodes must exist in the graph. Returns the assigned edge ID.
    pub fn add_edge(
        &mut self,
        from_node: i64,
        to_node: i64,
        bidirectional: bool,
        name: &str,
        metadata: Option<MetaValue>,
    ) -> GraphResult<i64> {
        if !self.nodes.contains_key(&from_node) {
            return Err(GraphError::EdgeNodeMissing(from_node));
        }
        if !self.nodes.contains_key(&to_node) {
            return Err(GraphError::EdgeNodeMissing(to_node));
        }

        let id = self.next_edge_id;
        let edge = Edge::new(id, from_node, to_node, bidirectional, name, metadata);
        self.edges.insert(id, edge);

        self.adjacency.entry(from_node).or_default().insert(id);
        self.adjacency.entry(to_node).or_default().insert(id);

        self.next_edge_id += 1;
        Ok(id)
    }

    /// Removes an edge from the graph.
    ///
    /// Returns the removed Edge, or an error if the edge does not exist.
    pub fn remove_edge(&mut self, edge_id: i64) -> GraphResult<Edge> {
        let edge = self
            .edges
            .remove(&edge_id)
            .ok_or(GraphError::EdgeNotFound(edge_id))?;

        if let Some(adj) = self.adjacency.get_mut(&edge.node_a) {
            adj.remove(&edge_id);
        }
        if let Some(adj) = self.adjacency.get_mut(&edge.node_b) {
            adj.remove(&edge_id);
        }

        Ok(edge)
    }

    /// Returns a reference to an edge by ID, or an error if not found.
    pub fn get_edge(&self, edge_id: i64) -> GraphResult<&Edge> {
        self.edges
            .get(&edge_id)
            .ok_or(GraphError::EdgeNotFound(edge_id))
    }

    /// Returns a mutable reference to an edge by ID, or an error if not found.
    pub fn get_edge_mut(&mut self, edge_id: i64) -> GraphResult<&mut Edge> {
        self.edges
            .get_mut(&edge_id)
            .ok_or(GraphError::EdgeNotFound(edge_id))
    }

    /// Returns true if an edge with the given ID exists.
    pub fn has_edge(&self, edge_id: i64) -> bool {
        self.edges.contains_key(&edge_id)
    }

    /// Returns the number of edges in the graph.
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    // ========================================================================
    // Query operations
    // ========================================================================

    /// Returns the IDs of all edges connected to the given node.
    pub fn edges_of(&self, node_id: i64) -> GraphResult<&HashSet<i64>> {
        self.adjacency
            .get(&node_id)
            .ok_or(GraphError::NodeNotFound(node_id))
    }

    /// Returns the IDs of all nodes connected to the given node,
    /// regardless of edge direction.
    pub fn neighbors(&self, node_id: i64) -> GraphResult<Vec<i64>> {
        let edge_ids = self.edges_of(node_id)?;
        let mut neighbor_set = HashSet::new();

        for &edge_id in edge_ids {
            if let Some(edge) = self.edges.get(&edge_id) {
                let other = if edge.node_a == node_id {
                    edge.node_b
                } else {
                    edge.node_a
                };
                if other != node_id {
                    neighbor_set.insert(other);
                }
            }
        }

        Ok(neighbor_set.into_iter().collect())
    }

    /// Returns the IDs of all nodes reachable from the given node,
    /// respecting edge direction (uses `allows_traversal`).
    pub fn directed_neighbors(&self, node_id: i64) -> GraphResult<Vec<i64>> {
        let edge_ids = self.edges_of(node_id)?;
        let mut neighbor_set = HashSet::new();

        for &edge_id in edge_ids {
            if let Some(edge) = self.edges.get(&edge_id) {
                let other = if edge.node_a == node_id {
                    edge.node_b
                } else {
                    edge.node_a
                };
                if other != node_id && edge.allows_traversal(node_id, other) {
                    neighbor_set.insert(other);
                }
            }
        }

        Ok(neighbor_set.into_iter().collect())
    }

    /// Returns true if any edge exists between the two given nodes.
    pub fn has_edge_between(&self, node_a: i64, node_b: i64) -> GraphResult<bool> {
        if !self.nodes.contains_key(&node_a) {
            return Err(GraphError::NodeNotFound(node_a));
        }
        if !self.nodes.contains_key(&node_b) {
            return Err(GraphError::NodeNotFound(node_b));
        }

        // Check the node with fewer edges for efficiency
        let check_node = if self.adjacency.get(&node_a).map_or(0, |s| s.len())
            <= self.adjacency.get(&node_b).map_or(0, |s| s.len())
        {
            node_a
        } else {
            node_b
        };

        if let Some(edge_ids) = self.adjacency.get(&check_node) {
            for &edge_id in edge_ids {
                if let Some(edge) = self.edges.get(&edge_id) {
                    if edge.connects_pair(node_a, node_b) {
                        return Ok(true);
                    }
                }
            }
        }

        Ok(false)
    }

    /// Returns all edges between two given nodes.
    pub fn edges_between(&self, node_a: i64, node_b: i64) -> GraphResult<Vec<&Edge>> {
        if !self.nodes.contains_key(&node_a) {
            return Err(GraphError::NodeNotFound(node_a));
        }
        if !self.nodes.contains_key(&node_b) {
            return Err(GraphError::NodeNotFound(node_b));
        }

        let check_node = if self.adjacency.get(&node_a).map_or(0, |s| s.len())
            <= self.adjacency.get(&node_b).map_or(0, |s| s.len())
        {
            node_a
        } else {
            node_b
        };

        let mut result = Vec::new();
        if let Some(edge_ids) = self.adjacency.get(&check_node) {
            for &edge_id in edge_ids {
                if let Some(edge) = self.edges.get(&edge_id) {
                    if edge.connects_pair(node_a, node_b) {
                        result.push(edge);
                    }
                }
            }
        }

        Ok(result)
    }

    /// Returns the degree of a node (number of edges connected to it).
    pub fn degree(&self, node_id: i64) -> GraphResult<usize> {
        self.adjacency
            .get(&node_id)
            .map(|set| set.len())
            .ok_or(GraphError::NodeNotFound(node_id))
    }

    // ========================================================================
    // Utility operations
    // ========================================================================

    /// Rebuilds the adjacency index from the edges HashMap.
    ///
    /// Useful after deserialization from an external source that
    /// does not include the adjacency index.
    pub fn rebuild_adjacency(&mut self) {
        self.adjacency.clear();

        for &node_id in self.nodes.keys() {
            self.adjacency.insert(node_id, HashSet::new());
        }

        for (&edge_id, edge) in &self.edges {
            self.adjacency
                .entry(edge.node_a)
                .or_default()
                .insert(edge_id);
            self.adjacency
                .entry(edge.node_b)
                .or_default()
                .insert(edge_id);
        }
    }
}

impl HasMetadata for Graph {
    fn metadata(&self) -> &MetaValue {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut MetaValue {
        &mut self.metadata
    }
}
