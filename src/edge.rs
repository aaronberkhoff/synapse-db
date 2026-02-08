use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::meta_data::{HasMetadata, MetaValue};

/// Represents the direction of an edge between two nodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EdgeDirection {
    /// Edge goes from node_a to node_b only.
    AtoB,
    /// Edge goes from node_b to node_a only.
    BtoA,
    /// Edge goes both directions.
    Bidirectional,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub id: i64,
    /// The node with the smaller ID.
    pub node_a: i64,
    /// The node with the larger ID.
    pub node_b: i64,
    /// The direction of the edge.
    pub direction: EdgeDirection,
    pub name: String,
    pub metadata: MetaValue,
}

impl Edge {
    /// Creates a new edge between two nodes.
    ///
    /// The node IDs are automatically sorted so that `node_a < node_b`.
    /// The direction is determined based on the original order of the IDs.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for this edge
    /// * `from_node` - The source node ID
    /// * `to_node` - The target node ID
    /// * `bidirectional` - If true, edge goes both ways; if false, only from → to
    /// * `name` - The name/label of this edge
    /// * `metadata` - Optional metadata
    pub fn new(
        id: i64,
        from_node: i64,
        to_node: i64,
        bidirectional: bool,
        name: &str,
        metadata: Option<MetaValue>,
    ) -> Self {
        let (node_a, node_b, direction) = if from_node <= to_node {
            let dir = if bidirectional {
                EdgeDirection::Bidirectional
            } else {
                EdgeDirection::AtoB
            };
            (from_node, to_node, dir)
        } else {
            let dir = if bidirectional {
                EdgeDirection::Bidirectional
            } else {
                EdgeDirection::BtoA
            };
            (to_node, from_node, dir)
        };

        Edge {
            id,
            node_a,
            node_b,
            direction,
            name: name.to_string(),
            metadata: metadata.unwrap_or(MetaValue::Map(HashMap::new())),
        }
    }

    /// Returns true if this edge connects the given node.
    pub fn connects(&self, node_id: i64) -> bool {
        self.node_a == node_id || self.node_b == node_id
    }

    /// Returns true if this edge connects both given nodes (in any order).
    pub fn connects_pair(&self, x: i64, y: i64) -> bool {
        let (a, b) = if x <= y { (x, y) } else { (y, x) };
        self.node_a == a && self.node_b == b
    }

    /// Returns true if traversal from `from` to `to` is allowed.
    pub fn allows_traversal(&self, from: i64, to: i64) -> bool {
        if !self.connects_pair(from, to) {
            return false;
        }
        match self.direction {
            EdgeDirection::Bidirectional => true,
            EdgeDirection::AtoB => from == self.node_a && to == self.node_b,
            EdgeDirection::BtoA => from == self.node_b && to == self.node_a,
        }
    }
}

impl HasMetadata for Edge {
    fn metadata(&self) -> &MetaValue {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut MetaValue {
        &mut self.metadata
    }
}
