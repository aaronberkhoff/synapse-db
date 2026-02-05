//! Python bindings for Synapse DB.
//!
//! This module provides Python bindings via PyO3, exposing the core
//! Synapse DB functionality to Python users.
//!
//! # Module Structure
//!
//! - `convert` - Type conversion utilities between Rust and Python
//! - `node` - Python wrapper for the `Node` struct
//! - `edge` - Python wrapper for the `Edge` struct
//!
//! # Example
//!
//! ```python
//! import synapse_db
//!
//! # Create nodes
//! node1 = synapse_db.Node(1, "alice")
//! node2 = synapse_db.Node(2, "bob")
//!
//! # Create an edge (directed: alice -> bob)
//! edge = synapse_db.Edge(1, 1, 2, "follows")
//!
//! # Create a bidirectional edge
//! friendship = synapse_db.Edge(2, 1, 2, "friends", bidirectional=True)
//!
//! # Check traversal
//! edge.allows_traversal(1, 2)  # True
//! edge.allows_traversal(2, 1)  # False (directed edge)
//!
//! # Set metadata
//! node1.set_meta("version", 1)
//! edge.set_meta("weight", 1.5)
//! ```

mod convert;
mod edge;
mod graph;
mod node;

use pyo3::prelude::*;
use pyo3_stub_gen::define_stub_info_gatherer;
use pyo3_stub_gen::derive::gen_stub_pyfunction;

pub use edge::{PyEdge, PyEdgeDirection};
pub use graph::PyGraph;
pub use node::PyNode;

/// Get the version of the synapse_db library.
///
/// Returns:
///     The version string (e.g., "0.1.0").
///
/// Example:
///     >>> import synapse_db
///     >>> synapse_db.get_version()
///     '0.1.0'
#[gen_stub_pyfunction]
#[pyfunction]
fn get_version() -> &'static str {
    crate::version()
}

/// Synapse DB - A high-performance database written in Rust.
///
/// This module provides Python bindings to the Synapse DB core functionality.
///
/// Classes:
///     Node: A node in the database hierarchy.
///     Edge: An edge connecting two nodes (with canonical ordering).
///     EdgeDirection: Direction enum (AtoB, BtoA, Bidirectional).
///     Graph: A graph containing nodes and edges with adjacency indexing.
///
/// Functions:
///     get_version(): Get the library version.
///
/// Example:
///     >>> import synapse_db
///     >>> node = synapse_db.Node(1, "root")
///     >>> edge = synapse_db.Edge(1, 2, 5, "follows")
///     >>> edge.allows_traversal(2, 5)
///     True
#[pymodule]
fn synapse_db(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyNode>()?;
    m.add_class::<PyEdge>()?;
    m.add_class::<PyEdgeDirection>()?;
    m.add_class::<PyGraph>()?;
    m.add_function(wrap_pyfunction!(get_version, m)?)?;
    Ok(())
}

// Define the stub info gatherer for generating .pyi files
define_stub_info_gatherer!(stub_info);

// Re-export for the stub_gen binary
pub use stub_info as get_stub_info;
