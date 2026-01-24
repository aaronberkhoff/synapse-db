"""Tests for module version and metadata."""

import synapse_db


class TestVersion:
    """Test cases for version information."""

    def test_version_function(self) -> None:
        """Test that get_version() returns a string."""
        ver = synapse_db.get_version()
        assert isinstance(ver, str)
        assert len(ver) > 0

    def test_version_format(self) -> None:
        """Test that version follows semver format."""
        ver = synapse_db.get_version()
        parts = ver.split(".")
        assert len(parts) >= 2  # At least major.minor
        assert all(part.isdigit() for part in parts[:2])

    def test_dunder_version(self) -> None:
        """Test that __version__ attribute is set."""
        assert hasattr(synapse_db, "__version__")
        assert synapse_db.__version__ == synapse_db.get_version()


class TestModuleExports:
    """Test cases for module exports."""

    def test_all_exports(self) -> None:
        """Test that __all__ contains expected exports."""
        expected = ["Node", "get_version"]
        assert set(expected) == set(synapse_db.__all__)

    def test_exports_are_accessible(self) -> None:
        """Test that all exports are accessible."""
        for name in synapse_db.__all__:
            assert hasattr(synapse_db, name)
