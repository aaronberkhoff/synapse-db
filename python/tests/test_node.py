"""Tests for the Node class."""

import json

import pytest

import synapse_db


class TestNodeCreation:
    """Test cases for Node creation."""

    def test_node_creation_basic(self) -> None:
        """Test that Node can be instantiated with minimal args."""
        node = synapse_db.Node(1, "test")
        assert node.id == 1
        assert node.name == "test"
        assert node.parent_id is None
        assert node.children_count == 0

    def test_node_creation_with_parent(self) -> None:
        """Test Node creation with parent_id."""
        node = synapse_db.Node(2, "child", parent_id=1)
        assert node.id == 2
        assert node.name == "child"
        assert node.parent_id == 1

    def test_node_creation_with_children_count(self) -> None:
        """Test Node creation with children_count."""
        node = synapse_db.Node(1, "parent", children_count=5)
        assert node.children_count == 5

    def test_node_creation_with_metadata(self) -> None:
        """Test Node creation with metadata."""
        metadata = {"version": 1, "name": "config"}
        node = synapse_db.Node(1, "test", metadata=metadata)
        assert node.metadata == metadata


class TestNodeProperties:
    """Test cases for Node properties."""

    def test_node_id_property(self, node: synapse_db.Node) -> None:
        """Test that id property returns correct value."""
        assert node.id == 1

    def test_node_name_property(self, node: synapse_db.Node) -> None:
        """Test that name property returns correct value."""
        assert node.name == "test_node"

    def test_node_parent_id_none(self, root_node: synapse_db.Node) -> None:
        """Test that parent_id is None for root nodes."""
        assert root_node.parent_id is None

    def test_node_parent_id_set(self, child_node: synapse_db.Node) -> None:
        """Test that parent_id is set for child nodes."""
        assert child_node.parent_id == 1

    def test_node_children_count(self, root_node: synapse_db.Node) -> None:
        """Test children_count property."""
        assert root_node.children_count == 2


class TestNodeMetadata:
    """Test cases for Node metadata operations."""

    def test_default_metadata_is_empty_dict(self) -> None:
        """Test that default metadata is an empty dict."""
        node = synapse_db.Node(1, "test")
        assert node.metadata == {}

    def test_set_meta_int(self) -> None:
        """Test setting integer metadata."""
        node = synapse_db.Node(1, "test")
        node.set_meta("count", 42)
        assert node.get_meta("count") == 42

    def test_set_meta_float(self) -> None:
        """Test setting float metadata."""
        node = synapse_db.Node(1, "test")
        node.set_meta("ratio", 3.14)
        assert node.get_meta("ratio") == 3.14

    def test_set_meta_string(self) -> None:
        """Test setting string metadata."""
        node = synapse_db.Node(1, "test")
        node.set_meta("label", "hello")
        assert node.get_meta("label") == "hello"

    def test_set_meta_list(self) -> None:
        """Test setting list metadata."""
        node = synapse_db.Node(1, "test")
        node.set_meta("tags", ["rust", "python", "database"])
        assert node.get_meta("tags") == ["rust", "python", "database"]

    def test_set_meta_dict(self) -> None:
        """Test setting dict metadata."""
        node = synapse_db.Node(1, "test")
        nested = {"key": "value", "number": 123}
        node.set_meta("config", nested)
        assert node.get_meta("config") == nested

    def test_set_meta_nested(self) -> None:
        """Test setting deeply nested metadata."""
        node = synapse_db.Node(1, "test")
        nested = {
            "level1": {
                "level2": {
                    "items": [1, 2, 3],
                    "name": "deep"
                }
            }
        }
        node.set_meta("nested", nested)
        result = node.get_meta("nested")
        assert result["level1"]["level2"]["name"] == "deep"

    def test_get_meta_nonexistent_key(self) -> None:
        """Test getting nonexistent metadata returns None."""
        node = synapse_db.Node(1, "test")
        assert node.get_meta("nonexistent") is None

    def test_metadata_overwrite(self) -> None:
        """Test that metadata can be overwritten."""
        node = synapse_db.Node(1, "test")
        node.set_meta("key", "original")
        node.set_meta("key", "updated")
        assert node.get_meta("key") == "updated"

    def test_multiple_metadata_keys(self) -> None:
        """Test setting multiple metadata keys."""
        node = synapse_db.Node(1, "test")
        node.set_meta("key1", 1)
        node.set_meta("key2", "two")
        node.set_meta("key3", [3])
        assert node.get_meta("key1") == 1
        assert node.get_meta("key2") == "two"
        assert node.get_meta("key3") == [3]


class TestNodeRepr:
    """Test cases for Node string representation."""

    def test_node_repr(self) -> None:
        """Test Node __repr__ method."""
        node = synapse_db.Node(1, "test")
        repr_str = repr(node)
        assert "Node" in repr_str
        assert "id=1" in repr_str
        assert "name='test'" in repr_str

    def test_node_repr_with_parent(self) -> None:
        """Test Node __repr__ with parent_id."""
        node = synapse_db.Node(2, "child", parent_id=1)
        repr_str = repr(node)
        assert "parent_id=Some(1)" in repr_str or "parent_id=1" in repr_str


class TestNodeJson:
    """Test cases for Node JSON serialization."""

    def test_to_json(self) -> None:
        """Test Node to_json method."""
        node = synapse_db.Node(1, "test")
        json_str = node.to_json()
        assert isinstance(json_str, str)
        data = json.loads(json_str)
        assert data["id"] == 1
        assert data["name"] == "test"

    def test_to_json_pretty(self) -> None:
        """Test Node to_json_pretty method."""
        node = synapse_db.Node(1, "test")
        json_str = node.to_json_pretty()
        assert isinstance(json_str, str)
        assert "\n" in json_str  # Pretty printed has newlines

    def test_from_json(self) -> None:
        """Test Node.from_json static method."""
        original = synapse_db.Node(1, "test")
        original.set_meta("key", "value")
        json_str = original.to_json()

        restored = synapse_db.Node.from_json(json_str)
        assert restored.id == original.id
        assert restored.name == original.name

    def test_from_json_invalid(self) -> None:
        """Test from_json with invalid JSON raises ValueError."""
        with pytest.raises(ValueError):
            synapse_db.Node.from_json("not valid json")

    def test_json_roundtrip(self) -> None:
        """Test JSON serialization roundtrip preserves data."""
        original = synapse_db.Node(
            id=42,
            name="roundtrip",
            parent_id=10,
            children_count=3,
            metadata={"tags": ["a", "b"], "count": 100}
        )

        json_str = original.to_json()
        restored = synapse_db.Node.from_json(json_str)

        assert restored.id == original.id
        assert restored.name == original.name
        assert restored.parent_id == original.parent_id
        assert restored.children_count == original.children_count


class TestNodeIntegration:
    """Integration tests for Node class."""

    def test_node_hierarchy(self) -> None:
        """Test creating a node hierarchy."""
        root = synapse_db.Node(1, "root", children_count=2)
        child1 = synapse_db.Node(2, "child1", parent_id=root.id)
        child2 = synapse_db.Node(3, "child2", parent_id=root.id)

        assert child1.parent_id == root.id
        assert child2.parent_id == root.id
        assert root.children_count == 2

    def test_multiple_node_instances(self) -> None:
        """Test creating multiple Node instances."""
        node1 = synapse_db.Node(1, "node1")
        node2 = synapse_db.Node(2, "node2")
        assert node1 is not node2
        assert node1.id != node2.id
