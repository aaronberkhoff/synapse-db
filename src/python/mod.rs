//! Python bindings for Synapse DB.
//!
//! This module provides Python bindings via PyO3, exposing the core
//! Synapse DB functionality to Python users.
//!
//! # Module Structure
//!
//! - `convert` - Type conversion utilities between Rust and Python
//! - `node` - Python wrapper for the `Node` struct
//!
//! # Example
//!
//! ```python
//! import synapse_db
//!
//! # Create a node
//! node = synapse_db.Node(1, "root")
//!
//! # Set metadata
//! node.set_meta("version", 1)
//! node.set_meta("tags", ["database", "rust"])
//!
//! # Get metadata
//! version = node.get_meta("version")
//!
//! # Serialize to JSON
//! json_str = node.to_json()
//! ```

mod convert;
mod node;

use pyo3::prelude::*;
use pyo3_stub_gen::define_stub_info_gatherer;
use pyo3_stub_gen::derive::gen_stub_pyfunction;

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
///
/// Functions:
///     get_version(): Get the library version.
///
/// Example:
///     >>> import synapse_db
///     >>> node = synapse_db.Node(1, "root")
///     >>> node.set_meta("version", 1)
///     >>> print(node.get_meta("version"))
///     1
#[pymodule]
fn synapse_db(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyNode>()?;
    m.add_function(wrap_pyfunction!(get_version, m)?)?;
    Ok(())
}

// Define the stub info gatherer for generating .pyi files
define_stub_info_gatherer!(stub_info);

// Re-export for the stub_gen binary
pub use stub_info as get_stub_info;
