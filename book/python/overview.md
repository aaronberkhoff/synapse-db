# Python Bindings

Synapse DB provides first-class Python bindings through [PyO3](https://pyo3.rs/), allowing you to use the high-performance Rust database engine directly from Python.

## Features

- **Native Performance**: The core database engine runs as compiled Rust code
- **Type Safety**: Full type annotations with `.pyi` stub files
- **Pythonic API**: Familiar Python interfaces and exceptions
- **Zero-Copy Where Possible**: Efficient data transfer between Rust and Python

## Quick Example

```python
import synapse_db

# Create a database instance
db = synapse_db.Database()

# Execute queries
db.execute("CREATE TABLE users (id INTEGER, name TEXT)")
db.execute("INSERT INTO users VALUES (1, 'Alice')")

# Check version
print(f"Synapse DB version: {synapse_db.version()}")
```

## Module Structure

The `synapse_db` module exposes:

| Class/Function | Description |
|----------------|-------------|
| `Database` | Main database interface combining storage and query |
| `Storage` | Low-level storage layer access |
| `QueryEngine` | Direct query execution engine |
| `version()` | Returns the library version string |

## Next Steps

- [Installation](./installation.md) - How to install the Python package
- [Usage Guide](./usage.md) - Detailed usage examples
- [API Reference](./api.md) - Complete API documentation
