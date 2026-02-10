//! Schema definitions for the infile parser.
//!
//! A `Schema` defines the valid structure of a graph database — what node types
//! and edge types are allowed, what properties each type requires or optionally
//! accepts, and which node types each edge type can connect.
//!
//! Schemas are validated on construction: if an edge type references a node type
//! that doesn't exist, the `Schema` will fail to build.
//!
//! # Example
//!
//! ```
//! use std::collections::HashMap;
//! use synapse_db::infile::schema::{Schema, NodeTypeDef, EdgeTypeDef, PropType};
//! use synapse_db::meta_data::MetaValue;
//!
//! let mut node_types = HashMap::new();
//! node_types.insert("person".to_string(), NodeTypeDef {
//!     required: HashMap::from([("name".to_string(), PropType::Str)]),
//!     optional: HashMap::new(),
//! });
//!
//! let mut edge_types = HashMap::new();
//! edge_types.insert("knows".to_string(), EdgeTypeDef {
//!     from: vec!["person".to_string()],
//!     to: vec!["person".to_string()],
//!     bidirectional: true,
//!     required: HashMap::new(),
//!     optional: HashMap::new(),
//! });
//!
//! let schema = Schema::new(
//!     "social",
//!     MetaValue::Map(HashMap::new()),
//!     node_types,
//!     edge_types,
//! ).unwrap();
//! ```

use std::collections::HashMap;

use crate::infile::error::SchemaValidationError;
use crate::meta_data::MetaValue;

/// The allowed data types for properties in a schema definition.
///
/// Each variant corresponds to a `MetaValue` variant that will be
/// validated against at data load time.
///
/// | Schema type | PropType variant   | Expected MetaValue        |
/// |-------------|--------------------|---------------------------|
/// | `"int"`     | `PropType::Int`    | `MetaValue::Int(i64)`     |
/// | `"float"`   | `PropType::Float`  | `MetaValue::Float(f64)`   |
/// | `"string"`  | `PropType::Str`    | `MetaValue::Str(String)`  |
/// | `"bool"`    | `PropType::Bool`   | `MetaValue::Bool(bool)`   |
/// | `"vector"`  | `PropType::Vector` | `MetaValue::Vec(..)`      |
#[derive(Debug, Clone, PartialEq)]
pub enum PropType {
    Int,
    Float,
    Str,
    Bool,
    Vector,
}

impl fmt::Display for PropType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PropType::Int => write!(f, "int"),
            PropType::Float => write!(f, "float"),
            PropType::Str => write!(f, "string"),
            PropType::Bool => write!(f, "bool"),
            PropType::Vector => write!(f, "vector"),
        }
    }
}

/// Defines the required and optional properties for a node type.
///
/// When data is loaded against a schema, every node of this type must
/// contain all `required` properties with the correct `PropType`, and
/// may contain any of the `optional` properties. Properties not listed
/// in either map are rejected.
#[derive(Debug, Clone)]
pub struct NodeTypeDef {
    /// Properties that must be present on every node of this type.
    pub required: HashMap<String, PropType>,
    /// Properties that may optionally be present on nodes of this type.
    pub optional: HashMap<String, PropType>,
}

/// Defines the structure of an edge type, including connection constraints
/// and properties.
///
/// The `from` and `to` fields restrict which node types this edge can
/// connect. For example, an `is_member` edge with `from: ["spacecraft"]`
/// and `to: ["team"]` will reject an edge from a `"weapon"` node to a
/// `"team"` node.
#[derive(Debug, Clone)]
pub struct EdgeTypeDef {
    /// Node types allowed as the source of this edge.
    pub from: Vec<String>,
    /// Node types allowed as the target of this edge.
    pub to: Vec<String>,
    /// Whether this edge type is bidirectional by default.
    pub bidirectional: bool,
    /// Properties that must be present on every edge of this type.
    pub required: HashMap<String, PropType>,
    /// Properties that may optionally be present on edges of this type.
    pub optional: HashMap<String, PropType>,
}

/// A validated schema defining the structure of a graph database.
///
/// A `Schema` specifies:
/// - The graph's name and metadata
/// - What node types are allowed and their property definitions
/// - What edge types are allowed, their property definitions, and which
///   node types they can connect
///
/// Schemas are validated at construction time — if an edge type references
/// a node type that doesn't exist in `node_types`, construction fails with
/// a `SchemaValidationError`.
#[derive(Debug, Clone)]
pub struct Schema {
    /// The name of the graph this schema defines.
    pub graph_name: String,
    /// Default metadata for the graph.
    pub graph_metadata: MetaValue,
    /// The allowed node types and their property definitions.
    pub node_types: HashMap<String, NodeTypeDef>,
    /// The allowed edge types, their property definitions, and connection constraints.
    pub edge_types: HashMap<String, EdgeTypeDef>,
}

impl Schema {
    /// Creates a new `Schema` and validates it.
    ///
    /// Validation checks that every node type referenced in edge type
    /// `from` and `to` lists exists in `node_types`. If validation fails,
    /// returns a `SchemaValidationError`.
    ///
    /// # Arguments
    ///
    /// * `graph_name` - The name of the graph this schema defines
    /// * `graph_metadata` - Default metadata for the graph
    /// * `node_types` - Map of node type names to their property definitions
    /// * `edge_types` - Map of edge type names to their definitions
    ///
    /// # Errors
    ///
    /// Returns `SchemaValidationError` if:
    /// - Any edge type `from` or `to` list references a node type not in `node_types`
    /// - Any edge type has an empty `from` or `to` list
    pub fn new(
        graph_name: &str,
        graph_metadata: MetaValue,
        node_types: HashMap<String, NodeTypeDef>,
        edge_types: HashMap<String, EdgeTypeDef>,
    ) -> Result<Self, SchemaValidationError> {
        let schema = Schema {
            graph_name: graph_name.to_string(),
            graph_metadata,
            node_types,
            edge_types,
        };
        schema.validate()?;
        Ok(schema)
    }

    /// Validates that all edge type `from`/`to` references point to
    /// existing node types.
    fn validate(&self) -> Result<(), SchemaValidationError> {
        for (edge_name, edge_def) in &self.edge_types {
            if edge_def.from.is_empty() {
                return Err(SchemaValidationError::EmptyFromList {
                    edge_type: edge_name.clone(),
                });
            }
            if edge_def.to.is_empty() {
                return Err(SchemaValidationError::EmptyToList {
                    edge_type: edge_name.clone(),
                });
            }

            for from_type in &edge_def.from {
                if !self.node_types.contains_key(from_type) {
                    return Err(SchemaValidationError::InvalidFromNodeType {
                        edge_type: edge_name.clone(),
                        node_type: from_type.clone(),
                    });
                }
            }
            for to_type in &edge_def.to {
                if !self.node_types.contains_key(to_type) {
                    return Err(SchemaValidationError::InvalidToNodeType {
                        edge_type: edge_name.clone(),
                        node_type: to_type.clone(),
                    });
                }
            }
        }
        Ok(())
    }

    /// Returns `true` if the given node type name exists in the schema.
    pub fn has_node_type(&self, name: &str) -> bool {
        self.node_types.contains_key(name)
    }

    /// Returns `true` if the given edge type name exists in the schema.
    pub fn has_edge_type(&self, name: &str) -> bool {
        self.edge_types.contains_key(name)
    }

    /// Returns the `NodeTypeDef` for the given type name, if it exists.
    pub fn get_node_type(&self, name: &str) -> Option<&NodeTypeDef> {
        self.node_types.get(name)
    }

    /// Returns the `EdgeTypeDef` for the given type name, if it exists.
    pub fn get_edge_type(&self, name: &str) -> Option<&EdgeTypeDef> {
        self.edge_types.get(name)
    }
}
