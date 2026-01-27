//! Synapse DB - A database implementation in Rust.
//!
//! This crate provides the core database functionality including
//! node management and flexible metadata storage.
//!
//! # Features
//!
//! - `python` - Enable Python bindings via PyO3
//!
//! # Example
//!
//! ```
//! use synapse_db::meta_data::{HasMetadata, MetaValue};
//! use synapse_db::node::Node;
//!
//! // Create a root node
//! let mut root = Node::new(1, None, 0, "root", None);
//!
//! // Add metadata
//! root.set_meta("version", MetaValue::Int(1));
//!
//! // Retrieve metadata
//! if let Some(MetaValue::Int(v)) = root.get_meta("version") {
//!     assert_eq!(*v, 1);
//! }
//! ```

pub mod edge;
pub mod meta_data;
pub mod node;

#[cfg(feature = "python")]
mod python;

/// Get the version of the synapse_db library.
///
/// # Example
///
/// ```
/// let version = synapse_db::version();
/// assert!(!version.is_empty());
/// ```
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(feature = "python")]
pub use python::get_stub_info as stub_info;
