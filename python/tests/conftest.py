"""Pytest configuration and fixtures for synapse_db tests."""

import pytest

import synapse_db


@pytest.fixture
def node() -> synapse_db.Node:
    """Create a fresh Node instance for testing."""
    return synapse_db.Node(1, "test_node")


@pytest.fixture
def root_node() -> synapse_db.Node:
    """Create a root node (no parent) for testing."""
    return synapse_db.Node(1, "root", children_count=2)


@pytest.fixture
def child_node() -> synapse_db.Node:
    """Create a child node with a parent reference for testing."""
    return synapse_db.Node(2, "child", parent_id=1)


@pytest.fixture
def node_with_metadata() -> synapse_db.Node:
    """Create a node with metadata for testing."""
    return synapse_db.Node(1, "config", metadata={"version": 1, "name": "test"})
