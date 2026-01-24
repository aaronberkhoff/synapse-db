"""Synapse DB - A high-performance database written in Rust.

This module provides Python bindings to the Synapse DB core functionality.

Example:
    >>> import synapse_db
    >>> node = synapse_db.Node(1, "root")
    >>> node.set_meta("version", 1)
    >>> print(node.get_meta("version"))
    1

Classes:
    Node: A node in the database hierarchy with flexible metadata storage.

Functions:
    get_version: Returns the library version string.
"""

from synapse_db.synapse_db import Node, get_version

__all__ = [
    "Node",
    "get_version",
]

__version__ = get_version()
