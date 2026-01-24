# Testing Python Bindings

This guide covers how to run and write tests for the Synapse DB Python bindings.

## Running Tests

### Prerequisites

First, build and install the development version:

```bash
# Install maturin
pip install maturin

# Build and install in development mode
maturin develop

# Install test dependencies
pip install pytest pytest-cov
```

### Run All Tests

```bash
pytest python/tests/
```

### Run with Verbose Output

```bash
pytest python/tests/ -v
```

### Run Specific Test File

```bash
pytest python/tests/test_database.py
```

### Run Specific Test

```bash
pytest python/tests/test_database.py::TestDatabase::test_database_creation
```

### Run with Coverage

```bash
pytest python/tests/ --cov=synapse_db --cov-report=html
```

Coverage report will be generated in `htmlcov/`.

## Test Structure

Tests are organized by component:

```
python/tests/
├── __init__.py
├── conftest.py          # Shared fixtures
├── test_storage.py      # Storage class tests
├── test_query_engine.py # QueryEngine class tests
├── test_database.py     # Database class tests
└── test_version.py      # Version and module tests
```

## Writing Tests

### Basic Test Structure

```python
import pytest
import synapse_db


class TestMyFeature:
    """Test cases for my feature."""

    def test_basic_functionality(self) -> None:
        """Test description."""
        db = synapse_db.Database()
        assert db is not None
```

### Using Fixtures

Fixtures are defined in `conftest.py`:

```python
def test_with_fixture(database: synapse_db.Database) -> None:
    """Test using the database fixture."""
    assert database is not None
```

Available fixtures:
- `storage` - Fresh `Storage` instance
- `query_engine` - Fresh `QueryEngine` instance
- `database` - Fresh `Database` instance

### Testing Exceptions

```python
def test_error_handling(database: synapse_db.Database) -> None:
    """Test that errors are raised correctly."""
    with pytest.raises(RuntimeError, match="error message"):
        database.execute("INVALID")
```

### Parameterized Tests

```python
import pytest


@pytest.mark.parametrize("query", [
    "SELECT 1",
    "SELECT 2",
    "SELECT 'test'",
])
def test_multiple_queries(database: synapse_db.Database, query: str) -> None:
    """Test various query formats."""
    # Test implementation
    pass
```

## Type Checking Tests

Type stubs are automatically generated from Rust code using `pyo3-stub-gen`. If you've modified the Rust bindings, regenerate stubs first:

```bash
./scripts/generate_stubs.sh
```

Then run mypy on test files:

```bash
mypy python/tests/
```

Ensure all test files have proper type annotations:

```python
def test_typed_function(database: synapse_db.Database) -> None:
    result: None = database.execute("SELECT 1")
    assert result is None
```

## Continuous Integration

Tests run automatically on CI. See `.github/workflows/` for configuration.

### Local CI Simulation

Run the full test suite as CI would:

```bash
# Build release version
maturin develop --release

# Run tests with coverage
pytest python/tests/ -v --cov=synapse_db

# Run type checking
mypy python/tests/

# Run linting
ruff check python/
```

## Test Categories

### Unit Tests

Test individual components in isolation:

```python
class TestStorageUnit:
    def test_storage_creation(self) -> None:
        storage = synapse_db.Storage()
        assert storage is not None
```

### Integration Tests

Test components working together:

```python
class TestDatabaseIntegration:
    def test_full_workflow(self, database: synapse_db.Database) -> None:
        # Test a complete workflow
        database.execute("CREATE TABLE test (id INTEGER)")
        database.execute("INSERT INTO test VALUES (1)")
        database.execute("SELECT * FROM test")
```

### Regression Tests

Add tests for bugs that have been fixed:

```python
class TestRegressions:
    def test_issue_123_fixed(self) -> None:
        """Regression test for issue #123."""
        # Reproduce the bug scenario
        # Assert it no longer occurs
        pass
```
