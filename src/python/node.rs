//! Python wrapper for the `Node` struct.
//!
//! This module provides the `PyNode` class for Python, wrapping the Rust `Node` type.

// Allow useless_conversion lint for PyO3's automatic error conversions via `?` operator
#![allow(clippy::useless_conversion)]

use crate::meta_data::HasMetadata;
use crate::node::Node;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

use super::convert::{metavalue_to_py, py_to_metavalue};

/// A node in the database hierarchy.
///
/// Each node has a unique identifier, an optional parent reference,
/// a count of its children, a name, and flexible metadata storage.
///
/// Example:
///     >>> node = Node(1, "root")
///     >>> node.set_meta("version", 1)
///     >>> print(node.get_meta("version"))
///     1
#[gen_stub_pyclass]
#[pyclass(name = "Node")]
pub struct PyNode {
    inner: Node,
}

#[gen_stub_pymethods]
#[pymethods]
impl PyNode {
    /// Create a new Node.
    ///
    /// Args:
    ///     id: Unique identifier for this node.
    ///     name: The name of this node.
    ///     parent_id: Optional ID of the parent node.
    ///     children_count: Number of direct children (default: 0).
    ///     metadata: Optional metadata (int, float, str, list, or dict).
    ///
    /// Returns:
    ///     A new Node instance.
    ///
    /// Example:
    ///     >>> node = Node(1, "root")
    ///     >>> child = Node(2, "child", parent_id=1)
    #[new]
    #[pyo3(signature = (id, name, parent_id=None, children_count=0, metadata=None))]
    fn new(
        id: i64,
        name: &str,
        parent_id: Option<i64>,
        children_count: u32,
        metadata: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<Self> {
        let meta = match metadata {
            Some(obj) => Some(py_to_metavalue(obj)?),
            None => None,
        };
        Ok(PyNode {
            inner: Node::new(id, parent_id, children_count, name, meta),
        })
    }

    /// The unique identifier of this node.
    #[getter]
    fn id(&self) -> i64 {
        self.inner.id
    }

    /// The name of this node.
    #[getter]
    fn name(&self) -> &str {
        &self.inner.name
    }

    /// The ID of the parent node, or None if this is a root node.
    #[getter]
    fn parent_id(&self) -> Option<i64> {
        self.inner.parent_id
    }

    /// The number of direct children this node has.
    #[getter]
    fn children_count(&self) -> u32 {
        self.inner.children_count
    }

    /// The metadata associated with this node.
    ///
    /// Returns the metadata as a Python object (int, float, str, list, or dict).
    #[getter]
    fn metadata(&self, py: Python<'_>) -> PyResult<PyObject> {
        metavalue_to_py(py, &self.inner.metadata)
    }

    /// Set a metadata value by key.
    ///
    /// If the current metadata is not a dict, it will be converted to one
    /// with the original value stored under the key "value".
    ///
    /// Args:
    ///     key: The key under which to store the value.
    ///     value: The value to store (int, float, str, list, or dict).
    ///
    /// Example:
    ///     >>> node = Node(1, "example")
    ///     >>> node.set_meta("count", 42)
    ///     >>> node.set_meta("tags", ["rust", "python"])
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
    ///     The value if found, or None if the key doesn't exist
    ///     or metadata is not a dict.
    ///
    /// Example:
    ///     >>> node = Node(1, "example")
    ///     >>> node.set_meta("version", 1)
    ///     >>> node.get_meta("version")
    ///     1
    ///     >>> node.get_meta("missing") is None
    ///     True
    fn get_meta(&self, py: Python<'_>, key: &str) -> PyResult<Option<PyObject>> {
        match self.inner.get_meta(key) {
            Some(value) => Ok(Some(metavalue_to_py(py, value)?)),
            None => Ok(None),
        }
    }

    /// Get a string representation of the node.
    fn __repr__(&self) -> String {
        format!(
            "Node(id={}, name='{}', parent_id={:?}, children_count={})",
            self.inner.id, self.inner.name, self.inner.parent_id, self.inner.children_count
        )
    }

    /// Convert the node to a JSON string.
    ///
    /// Returns:
    ///     A JSON string representation of the node.
    ///
    /// Example:
    ///     >>> node = Node(1, "root")
    ///     >>> json_str = node.to_json()
    fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(&self.inner)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    }

    /// Convert the node to a pretty-printed JSON string.
    ///
    /// Returns:
    ///     A formatted JSON string representation of the node.
    fn to_json_pretty(&self) -> PyResult<String> {
        serde_json::to_string_pretty(&self.inner)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    }

    /// Create a Node from a JSON string.
    ///
    /// Args:
    ///     json_str: A JSON string representing a node.
    ///
    /// Returns:
    ///     A new Node instance.
    ///
    /// Raises:
    ///     ValueError: If the JSON string is invalid.
    ///
    /// Example:
    ///     >>> json_str = '{"id": 1, "name": "root", ...}'
    ///     >>> node = Node.from_json(json_str)
    #[staticmethod]
    fn from_json(json_str: &str) -> PyResult<Self> {
        let inner: Node = serde_json::from_str(json_str)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        Ok(PyNode { inner })
    }
}
