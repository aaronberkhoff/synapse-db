# Usage Guide

This guide covers common usage patterns for Synapse DB Python bindings.

## Basic Usage

### Creating a Database

The `Database` class is the main entry point:

```python
import synapse_db

# Create a new database instance
db = synapse_db.Database()
```

### Executing Queries

Use the `execute()` method to run queries:

```python
db = synapse_db.Database()

# Create a table
db.execute("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, email TEXT)")

# Insert data
db.execute("INSERT INTO users (id, name, email) VALUES (1, 'Alice', 'alice@example.com')")
db.execute("INSERT INTO users (id, name, email) VALUES (2, 'Bob', 'bob@example.com')")

# Query data
db.execute("SELECT * FROM users WHERE id = 1")
```

## Error Handling

Query errors raise `RuntimeError`:

```python
import synapse_db

db = synapse_db.Database()

try:
    db.execute("INVALID SQL QUERY")
except RuntimeError as e:
    print(f"Query failed: {e}")
```

## Low-Level Access

For advanced use cases, you can access the storage and query layers directly:

### Storage Layer

```python
import synapse_db

# Direct storage access
storage = synapse_db.Storage()
```

### Query Engine

```python
import synapse_db

# Direct query engine access
engine = synapse_db.QueryEngine()
engine.execute("SELECT 1")
```

## Type Checking

Synapse DB includes type stubs for full IDE support. With mypy:

```python
# This will type-check correctly
import synapse_db

def get_database() -> synapse_db.Database:
    return synapse_db.Database()

def run_query(db: synapse_db.Database, query: str) -> None:
    db.execute(query)
```

Run mypy:

```bash
mypy your_script.py
```

## Context Managers (Future)

Future versions will support context managers for automatic cleanup:

```python
# Coming soon
with synapse_db.Database() as db:
    db.execute("SELECT * FROM users")
# Automatically closed
```

## Thread Safety

Each `Database`, `Storage`, and `QueryEngine` instance is independent. For multi-threaded applications, create separate instances per thread or use appropriate synchronization.

```python
import threading
import synapse_db

def worker():
    # Each thread gets its own database instance
    db = synapse_db.Database()
    db.execute("SELECT 1")

threads = [threading.Thread(target=worker) for _ in range(4)]
for t in threads:
    t.start()
for t in threads:
    t.join()
```

## Best Practices

1. **Reuse Database Instances**: Create one `Database` per logical connection
2. **Handle Errors**: Always wrap queries in try/except blocks
3. **Use Type Hints**: Leverage the provided type stubs for better code quality
4. **Check Version Compatibility**: Verify version matches your expectations

```python
import synapse_db

# Check version at startup
required_version = "0.1.0"
if synapse_db.version() != required_version:
    print(f"Warning: Expected version {required_version}, got {synapse_db.version()}")
```
