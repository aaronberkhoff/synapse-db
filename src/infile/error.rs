//! Error types for the infile parser.
//!
//! `SchemaValidationError` covers errors found when constructing a `Schema`
//! (e.g., an edge type referencing a node type that doesn't exist).

use std::fmt;

/// Errors that can occur during schema validation.
#[derive(Debug, Clone)]
pub enum SchemaValidationError {
    /// An edge type's `from` list references a node type that doesn't exist.
    InvalidFromNodeType {
        edge_type: String,
        node_type: String,
    },
    /// An edge type's `to` list references a node type that doesn't exist.
    InvalidToNodeType {
        edge_type: String,
        node_type: String,
    },
    /// An edge type has an empty `from` list.
    EmptyFromList { edge_type: String },
    /// An edge type has an empty `to` list.
    EmptyToList { edge_type: String },
}

impl fmt::Display for SchemaValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SchemaValidationError::InvalidFromNodeType {
                edge_type,
                node_type,
            } => write!(
                f,
                "Edge type '{}' references unknown node type '{}' in 'from' list",
                edge_type, node_type
            ),
            SchemaValidationError::InvalidToNodeType {
                edge_type,
                node_type,
            } => write!(
                f,
                "Edge type '{}' references unknown node type '{}' in 'to' list",
                edge_type, node_type
            ),
            SchemaValidationError::EmptyFromList { edge_type } => {
                write!(f, "Edge type '{}' has an empty 'from' list", edge_type)
            }
            SchemaValidationError::EmptyToList { edge_type } => {
                write!(f, "Edge type '{}' has an empty 'to' list", edge_type)
            }
        }
    }
}

impl std::error::Error for SchemaValidationError {}
