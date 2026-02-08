//! Python wrapper for the `Graph` struct.
//!
//! This module provides the `PyGraph` class for Python, wrapping the Rust `Graph` type.

#![allow(clippy::useless_conversion)]

use crate::graph::{Graph, GraphError};
use crate::meta_data::HasMetadata;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

use super::convert::{metavalue_to_py, py_to_metavalue};
use super::edge::PyEdge;
use super::node::PyNode;

/// Converts a GraphError into a Python ValueError.
fn graph_err_to_py(e: GraphError) -> PyErr {
    PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string())
}

/// A graph containing nodes and edges with an adjacency index.
///
/// The graph auto-assigns IDs to nodes and edges, maintains an
/// adjacency index for fast neighbor lookups, and supports both
/// directed and bidirectional edges.
///
/// Example:
///     >>> g = Graph("social")
///     >>> alice = g.add_node("alice")
///     >>> bob = g.add_node("bob")
///     >>> g.add_edge(alice, bob, "follows")
///     1
///     >>> g.neighbors(alice)
///     [2]
#[gen_stub_pyclass]
#[pyclass(name = "Graph")]
pub struct PyGraph {
    inner: Graph,
}

#[gen_stub_pymethods]
#[pymethods]
impl PyGraph {
    /// Create a new empty Graph.
    ///
    /// Args:
    ///     name: The name of the graph.
    ///     metadata: Optional metadata (int, float, str, list, or dict).
    ///
    /// Returns:
    ///     A new Graph instance.
    ///
    /// Example:
    ///     >>> g = Graph("my_graph")
    ///     >>> g = Graph("my_graph", metadata={"version": 1})
    #[new]
    #[pyo3(signature = (name, metadata=None))]
    fn new(name: &str, metadata: Option<&Bound<'_, PyAny>>) -> PyResult<Self> {
        let meta = match metadata {
            Some(obj) => Some(py_to_metavalue(obj)?),
            None => None,
        };
        Ok(PyGraph {
            inner: Graph::new(name, meta),
        })
    }

    /// The name of the graph.
    #[getter]
    fn name(&self) -> &str {
        &self.inner.name
    }

    /// The metadata associated with this graph.
    #[getter]
    fn metadata(&self, py: Python<'_>) -> PyResult<PyObject> {
        metavalue_to_py(py, &self.inner.metadata)
    }

    /// The number of nodes in the graph.
    #[getter]
    fn node_count(&self) -> usize {
        self.inner.node_count()
    }

    /// The number of edges in the graph.
    #[getter]
    fn edge_count(&self) -> usize {
        self.inner.edge_count()
    }

    // ========================================================================
    // Node operations
    // ========================================================================

    /// Add a new node to the graph.
    ///
    /// Args:
    ///     name: The name of the node.
    ///     metadata: Optional metadata (int, float, str, list, or dict).
    ///
    /// Returns:
    ///     The auto-assigned node ID.
    ///
    /// Example:
    ///     >>> g = Graph("test")
    ///     >>> node_id = g.add_node("alice")
    #[pyo3(signature = (name, metadata=None))]
    fn add_node(&mut self, name: &str, metadata: Option<&Bound<'_, PyAny>>) -> PyResult<i64> {
        let meta = match metadata {
            Some(obj) => Some(py_to_metavalue(obj)?),
            None => None,
        };
        Ok(self.inner.add_node(name, meta))
    }

    /// Remove a node and all its connected edges from the graph.
    ///
    /// Args:
    ///     node_id: The ID of the node to remove.
    ///
    /// Returns:
    ///     The removed Node.
    ///
    /// Raises:
    ///     ValueError: If the node does not exist.
    fn remove_node(&mut self, node_id: i64) -> PyResult<PyNode> {
        let node = self.inner.remove_node(node_id).map_err(graph_err_to_py)?;
        Ok(PyNode::from_inner(node))
    }

    /// Get a node by ID.
    ///
    /// Args:
    ///     node_id: The ID of the node.
    ///
    /// Returns:
    ///     A copy of the Node.
    ///
    /// Raises:
    ///     ValueError: If the node does not exist.
    fn get_node(&self, node_id: i64) -> PyResult<PyNode> {
        let node = self.inner.get_node(node_id).map_err(graph_err_to_py)?;
        Ok(PyNode::from_inner(node.clone()))
    }

    /// Check if a node exists in the graph.
    ///
    /// Args:
    ///     node_id: The ID of the node to check.
    ///
    /// Returns:
    ///     True if the node exists.
    fn has_node(&self, node_id: i64) -> bool {
        self.inner.has_node(node_id)
    }

    // ========================================================================
    // Edge operations
    // ========================================================================

    /// Add a new edge between two nodes.
    ///
    /// Both nodes must exist in the graph. Node IDs are automatically
    /// sorted for canonical ordering.
    ///
    /// Args:
    ///     from_node: The source node ID.
    ///     to_node: The target node ID.
    ///     name: The name/label of the edge.
    ///     bidirectional: If True, edge goes both ways (default: False).
    ///     metadata: Optional metadata (int, float, str, list, or dict).
    ///
    /// Returns:
    ///     The auto-assigned edge ID.
    ///
    /// Raises:
    ///     ValueError: If either node does not exist.
    ///
    /// Example:
    ///     >>> g = Graph("test")
    ///     >>> a = g.add_node("alice")
    ///     >>> b = g.add_node("bob")
    ///     >>> g.add_edge(a, b, "follows")
    ///     1
    #[pyo3(signature = (from_node, to_node, name, bidirectional=false, metadata=None))]
    fn add_edge(
        &mut self,
        from_node: i64,
        to_node: i64,
        name: &str,
        bidirectional: bool,
        metadata: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<i64> {
        let meta = match metadata {
            Some(obj) => Some(py_to_metavalue(obj)?),
            None => None,
        };
        self.inner
            .add_edge(from_node, to_node, bidirectional, name, meta)
            .map_err(graph_err_to_py)
    }

    /// Remove an edge from the graph.
    ///
    /// Args:
    ///     edge_id: The ID of the edge to remove.
    ///
    /// Returns:
    ///     The removed Edge.
    ///
    /// Raises:
    ///     ValueError: If the edge does not exist.
    fn remove_edge(&mut self, edge_id: i64) -> PyResult<PyEdge> {
        let edge = self.inner.remove_edge(edge_id).map_err(graph_err_to_py)?;
        Ok(PyEdge::from_inner(edge))
    }

    /// Get an edge by ID.
    ///
    /// Args:
    ///     edge_id: The ID of the edge.
    ///
    /// Returns:
    ///     A copy of the Edge.
    ///
    /// Raises:
    ///     ValueError: If the edge does not exist.
    fn get_edge(&self, edge_id: i64) -> PyResult<PyEdge> {
        let edge = self.inner.get_edge(edge_id).map_err(graph_err_to_py)?;
        Ok(PyEdge::from_inner(edge.clone()))
    }

    /// Check if an edge exists in the graph.
    ///
    /// Args:
    ///     edge_id: The ID of the edge to check.
    ///
    /// Returns:
    ///     True if the edge exists.
    fn has_edge(&self, edge_id: i64) -> bool {
        self.inner.has_edge(edge_id)
    }

    // ========================================================================
    // Query operations
    // ========================================================================

    /// Get all edge IDs connected to a node.
    ///
    /// Args:
    ///     node_id: The ID of the node.
    ///
    /// Returns:
    ///     A list of edge IDs.
    ///
    /// Raises:
    ///     ValueError: If the node does not exist.
    fn edges_of(&self, node_id: i64) -> PyResult<Vec<i64>> {
        let edges = self.inner.edges_of(node_id).map_err(graph_err_to_py)?;
        Ok(edges.iter().copied().collect())
    }

    /// Get all node IDs connected to a node, regardless of edge direction.
    ///
    /// Args:
    ///     node_id: The ID of the node.
    ///
    /// Returns:
    ///     A list of neighbor node IDs.
    ///
    /// Raises:
    ///     ValueError: If the node does not exist.
    ///
    /// Example:
    ///     >>> g = Graph("test")
    ///     >>> a = g.add_node("alice")
    ///     >>> b = g.add_node("bob")
    ///     >>> g.add_edge(a, b, "follows")
    ///     1
    ///     >>> g.neighbors(a)
    ///     [2]
    fn neighbors(&self, node_id: i64) -> PyResult<Vec<i64>> {
        self.inner.neighbors(node_id).map_err(graph_err_to_py)
    }

    /// Get all node IDs reachable from a node, respecting edge direction.
    ///
    /// Args:
    ///     node_id: The ID of the node.
    ///
    /// Returns:
    ///     A list of reachable neighbor node IDs.
    ///
    /// Raises:
    ///     ValueError: If the node does not exist.
    fn directed_neighbors(&self, node_id: i64) -> PyResult<Vec<i64>> {
        self.inner
            .directed_neighbors(node_id)
            .map_err(graph_err_to_py)
    }

    /// Check if any edge exists between two nodes.
    ///
    /// Args:
    ///     node_a: First node ID.
    ///     node_b: Second node ID.
    ///
    /// Returns:
    ///     True if any edge connects the two nodes.
    ///
    /// Raises:
    ///     ValueError: If either node does not exist.
    fn has_edge_between(&self, node_a: i64, node_b: i64) -> PyResult<bool> {
        self.inner
            .has_edge_between(node_a, node_b)
            .map_err(graph_err_to_py)
    }

    /// Get the degree of a node (number of edges connected to it).
    ///
    /// Args:
    ///     node_id: The ID of the node.
    ///
    /// Returns:
    ///     The number of edges connected to the node.
    ///
    /// Raises:
    ///     ValueError: If the node does not exist.
    fn degree(&self, node_id: i64) -> PyResult<usize> {
        self.inner.degree(node_id).map_err(graph_err_to_py)
    }

    // ========================================================================
    // Metadata operations
    // ========================================================================

    /// Set a metadata value by key on the graph.
    ///
    /// Args:
    ///     key: The key under which to store the value.
    ///     value: The value to store (int, float, str, list, or dict).
    fn set_meta(&mut self, key: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
        let meta_value = py_to_metavalue(value)?;
        self.inner.set_meta(key, meta_value);
        Ok(())
    }

    /// Get a metadata value by key from the graph.
    ///
    /// Args:
    ///     key: The key to look up.
    ///
    /// Returns:
    ///     The value if found, or None if the key doesn't exist.
    fn get_meta(&self, py: Python<'_>, key: &str) -> PyResult<Option<PyObject>> {
        match self.inner.get_meta(key) {
            Some(value) => Ok(Some(metavalue_to_py(py, value)?)),
            None => Ok(None),
        }
    }

    // ========================================================================
    // Serialization
    // ========================================================================

    /// Get a string representation of the graph.
    fn __repr__(&self) -> String {
        format!(
            "Graph(name='{}', nodes={}, edges={})",
            self.inner.name,
            self.inner.node_count(),
            self.inner.edge_count()
        )
    }

    /// Convert the graph to a JSON string.
    fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(&self.inner)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    }

    /// Convert the graph to a pretty-printed JSON string.
    fn to_json_pretty(&self) -> PyResult<String> {
        serde_json::to_string_pretty(&self.inner)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    }

    /// Create a Graph from a JSON string.
    ///
    /// Args:
    ///     json_str: A JSON string representing a graph.
    ///
    /// Returns:
    ///     A new Graph instance.
    ///
    /// Raises:
    ///     ValueError: If the JSON string is invalid.
    #[staticmethod]
    fn from_json(json_str: &str) -> PyResult<Self> {
        let inner: Graph = serde_json::from_str(json_str)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        Ok(PyGraph { inner })
    }
}
