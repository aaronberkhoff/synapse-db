//! Python wrapper for the `Edge` struct.
//!
//! This module provides the `PyEdge` class for Python, wrapping the Rust `Edge` type.

#![allow(clippy::useless_conversion)]

use crate::edge::{Edge, EdgeDirection};
use crate::meta_data::HasMetadata;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};
use pyo3_stub_gen::{PyStubType, TypeInfo};

use super::convert::{metavalue_to_py, py_to_metavalue};

/// Represents the direction of an edge between two nodes.
///
/// Variants:
///     AtoB: Edge goes from node_a to node_b only.
///     BtoA: Edge goes from node_b to node_a only.
///     Bidirectional: Edge goes both directions.
#[pyclass(name = "EdgeDirection", eq, eq_int)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PyEdgeDirection {
    /// Edge goes from node_a to node_b only.
    AtoB = 0,
    /// Edge goes from node_b to node_a only.
    BtoA = 1,
    /// Edge goes both directions.
    Bidirectional = 2,
}

impl PyStubType for PyEdgeDirection {
    fn type_output() -> TypeInfo {
        TypeInfo::with_module("synapse_db.EdgeDirection", "synapse_db".into())
    }
}

impl From<EdgeDirection> for PyEdgeDirection {
    fn from(dir: EdgeDirection) -> Self {
        match dir {
            EdgeDirection::AtoB => PyEdgeDirection::AtoB,
            EdgeDirection::BtoA => PyEdgeDirection::BtoA,
            EdgeDirection::Bidirectional => PyEdgeDirection::Bidirectional,
        }
    }
}

impl From<PyEdgeDirection> for EdgeDirection {
    fn from(dir: PyEdgeDirection) -> Self {
        match dir {
            PyEdgeDirection::AtoB => EdgeDirection::AtoB,
            PyEdgeDirection::BtoA => EdgeDirection::BtoA,
            PyEdgeDirection::Bidirectional => EdgeDirection::Bidirectional,
        }
    }
}

/// An edge connecting two nodes in the graph.
///
/// Edges use canonical ordering: node_a always has the smaller ID.
/// The direction field tracks the logical direction of the edge.
///
/// Example:
///     >>> edge = Edge(1, 5, 2, "follows")  # Creates edge from node 5 to node 2
///     >>> edge.node_a  # Always the smaller ID
///     2
///     >>> edge.node_b  # Always the larger ID
///     5
///     >>> edge.direction  # BtoA because 5 > 2, so original direction was B to A
///     EdgeDirection.BtoA
#[gen_stub_pyclass]
#[pyclass(name = "Edge")]
pub struct PyEdge {
    inner: Edge,
}

impl PyEdge {
    /// Creates a PyEdge from an existing Edge.
    pub(crate) fn from_inner(edge: Edge) -> Self {
        PyEdge { inner: edge }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyEdge {
    /// Create a new Edge between two nodes.
    ///
    /// The node IDs are automatically sorted so that node_a < node_b.
    /// The direction is determined based on the original order of the IDs.
    ///
    /// Args:
    ///     id: Unique identifier for this edge.
    ///     from_node: The source node ID.
    ///     to_node: The target node ID.
    ///     name: The name/label of this edge.
    ///     bidirectional: If True, edge goes both ways (default: False).
    ///     metadata: Optional metadata (int, float, str, list, or dict).
    ///
    /// Returns:
    ///     A new Edge instance.
    ///
    /// Example:
    ///     >>> edge = Edge(1, 5, 2, "follows")
    ///     >>> bidirectional_edge = Edge(2, 1, 3, "friends", bidirectional=True)
    #[new]
    #[pyo3(signature = (id, from_node, to_node, name, bidirectional=false, metadata=None))]
    fn new(
        id: i64,
        from_node: i64,
        to_node: i64,
        name: &str,
        bidirectional: bool,
        metadata: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<Self> {
        let meta = match metadata {
            Some(obj) => Some(py_to_metavalue(obj)?),
            None => None,
        };
        Ok(PyEdge {
            inner: Edge::new(id, from_node, to_node, bidirectional, name, meta),
        })
    }

    /// The unique identifier of this edge.
    #[getter]
    fn id(&self) -> i64 {
        self.inner.id
    }

    /// The node with the smaller ID (canonical ordering).
    #[getter]
    fn node_a(&self) -> i64 {
        self.inner.node_a
    }

    /// The node with the larger ID (canonical ordering).
    #[getter]
    fn node_b(&self) -> i64 {
        self.inner.node_b
    }

    /// The direction of the edge.
    #[getter]
    fn direction(&self) -> PyEdgeDirection {
        self.inner.direction.into()
    }

    /// The name/label of this edge.
    #[getter]
    fn name(&self) -> &str {
        &self.inner.name
    }

    /// The metadata associated with this edge.
    #[getter]
    fn metadata(&self, py: Python<'_>) -> PyResult<PyObject> {
        metavalue_to_py(py, &self.inner.metadata)
    }

    /// Set a metadata value by key.
    ///
    /// Args:
    ///     key: The key under which to store the value.
    ///     value: The value to store (int, float, str, list, or dict).
    ///
    /// Example:
    ///     >>> edge = Edge(1, 2, 3, "link")
    ///     >>> edge.set_meta("weight", 1.5)
    fn set_meta(&mut self, key: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
        let meta_value = py_to_metavalue(value)?;
        self.inner.set_meta(key, meta_value);
        Ok(())
    }

    /// Get a metadata value by key.
    ///
    /// Args:
    ///     key: The key to look up.
    ///
    /// Returns:
    ///     The value if found, or None if the key doesn't exist.
    ///
    /// Example:
    ///     >>> edge.set_meta("weight", 1.5)
    ///     >>> edge.get_meta("weight")
    ///     1.5
    fn get_meta(&self, py: Python<'_>, key: &str) -> PyResult<Option<PyObject>> {
        match self.inner.get_meta(key) {
            Some(value) => Ok(Some(metavalue_to_py(py, value)?)),
            None => Ok(None),
        }
    }

    /// Check if this edge connects the given node.
    ///
    /// Args:
    ///     node_id: The node ID to check.
    ///
    /// Returns:
    ///     True if this edge connects to the given node.
    ///
    /// Example:
    ///     >>> edge = Edge(1, 2, 5, "link")
    ///     >>> edge.connects(2)
    ///     True
    ///     >>> edge.connects(3)
    ///     False
    fn connects(&self, node_id: i64) -> bool {
        self.inner.connects(node_id)
    }

    /// Check if this edge connects both given nodes.
    ///
    /// Args:
    ///     x: First node ID.
    ///     y: Second node ID.
    ///
    /// Returns:
    ///     True if this edge connects both nodes (in any order).
    ///
    /// Example:
    ///     >>> edge = Edge(1, 2, 5, "link")
    ///     >>> edge.connects_pair(2, 5)
    ///     True
    ///     >>> edge.connects_pair(5, 2)  # Order doesn't matter
    ///     True
    fn connects_pair(&self, x: i64, y: i64) -> bool {
        self.inner.connects_pair(x, y)
    }

    /// Check if traversal from one node to another is allowed.
    ///
    /// Args:
    ///     from_node: The starting node ID.
    ///     to_node: The target node ID.
    ///
    /// Returns:
    ///     True if traversal is allowed based on edge direction.
    ///
    /// Example:
    ///     >>> edge = Edge(1, 2, 5, "follows")  # Directed: 2 -> 5
    ///     >>> edge.allows_traversal(2, 5)
    ///     True
    ///     >>> edge.allows_traversal(5, 2)
    ///     False
    fn allows_traversal(&self, from_node: i64, to_node: i64) -> bool {
        self.inner.allows_traversal(from_node, to_node)
    }

    /// Get a string representation of the edge.
    fn __repr__(&self) -> String {
        format!(
            "Edge(id={}, node_a={}, node_b={}, direction={:?}, name='{}')",
            self.inner.id,
            self.inner.node_a,
            self.inner.node_b,
            self.inner.direction,
            self.inner.name
        )
    }

    /// Convert the edge to a JSON string.
    fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(&self.inner)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    }

    /// Convert the edge to a pretty-printed JSON string.
    fn to_json_pretty(&self) -> PyResult<String> {
        serde_json::to_string_pretty(&self.inner)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    }

    /// Create an Edge from a JSON string.
    ///
    /// Args:
    ///     json_str: A JSON string representing an edge.
    ///
    /// Returns:
    ///     A new Edge instance.
    ///
    /// Raises:
    ///     ValueError: If the JSON string is invalid.
    #[staticmethod]
    fn from_json(json_str: &str) -> PyResult<Self> {
        let inner: Edge = serde_json::from_str(json_str)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        Ok(PyEdge { inner })
    }
}
