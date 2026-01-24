# Python API Reference

Complete API reference for the `synapse_db` Python module.

## Module: synapse_db

```python
import synapse_db
```

### Functions

#### `version() -> str`

Returns the version string of the Synapse DB library.

**Returns**: Version string (e.g., `"0.1.0"`)

**Example**:
```python
>>> synapse_db.version()
'0.1.0'
```

---

### Classes

## `Database`

Main database interface combining storage and query capabilities.

```python
class Database:
    def __init__(self) -> None: ...
    def execute(self, query: str) -> None: ...
```

### Constructor

#### `Database()`

Creates a new database instance.

**Example**:
```python
db = synapse_db.Database()
```

### Methods

#### `execute(query: str) -> None`

Executes a query on the database.

**Parameters**:
- `query` (str): The SQL query string to execute

**Raises**:
- `RuntimeError`: If query execution fails

**Example**:
```python
db = synapse_db.Database()
db.execute("CREATE TABLE users (id INTEGER, name TEXT)")
db.execute("INSERT INTO users VALUES (1, 'Alice')")
```

---

## `Storage`

Low-level storage layer for data persistence.

```python
class Storage:
    def __init__(self) -> None: ...
```

### Constructor

#### `Storage()`

Creates a new storage instance.

**Example**:
```python
storage = synapse_db.Storage()
```

---

## `QueryEngine`

Query processing and execution engine.

```python
class QueryEngine:
    def __init__(self) -> None: ...
    def execute(self, query: str) -> None: ...
```

### Constructor

#### `QueryEngine()`

Creates a new query engine instance.

**Example**:
```python
engine = synapse_db.QueryEngine()
```

### Methods

#### `execute(query: str) -> None`

Executes a query string.

**Parameters**:
- `query` (str): The query string to execute

**Raises**:
- `RuntimeError`: If query execution fails

**Example**:
```python
engine = synapse_db.QueryEngine()
try:
    engine.execute("SELECT * FROM users")
except RuntimeError as e:
    print(f"Query failed: {e}")
```

---

## Module Attributes

### `__version__: str`

The version string, equivalent to calling `version()`.

```python
>>> synapse_db.__version__
'0.1.0'
```

### `__all__: list[str]`

List of public symbols exported by the module.

```python
>>> synapse_db.__all__
['Database', 'QueryEngine', 'Storage', 'version']
```

---

## Exceptions

All Synapse DB operations that can fail raise standard Python exceptions:

| Exception | When Raised |
|-----------|-------------|
| `RuntimeError` | Query execution failures |
| `TypeError` | Invalid argument types |
| `ValueError` | Invalid argument values |

---

## Type Stubs

Full type information is available via the `synapse_db.pyi` stub file. This enables:

- IDE autocompletion
- Static type checking with mypy
- Better documentation in IDEs

Example mypy configuration (`pyproject.toml`):

```toml
[tool.mypy]
python_version = "3.9"
strict = true
```
