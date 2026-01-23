# User Guide

Comprehensive guide to using Synapse DB effectively.

## Architecture

Synapse DB consists of two main components:

```
┌─────────────────────────────────────┐
│      Client Application             │
└──────────────┬──────────────────────┘
               │ SQL Queries
┌──────────────▼──────────────────────┐
│         Query Engine                │
│  • Parse SQL queries                │
│  • Optimize execution plan          │
│  • Execute operations               │
└──────────────┬──────────────────────┘
               │ Read/Write
┌──────────────▼──────────────────────┐
│        Storage Layer                │
│  • Persist data to disk             │
│  • Manage indexes                   │
│  • Handle caching                   │
└─────────────────────────────────────┘
```

## Core Components

### [Storage Layer](./storage.md)

Handles data persistence and retrieval:

- Data structures for efficient storage
- Index management
- Cache layer
- Transaction support (planned)

### [Query Engine](./query-engine.md)

Processes and executes queries:

- SQL parsing
- Query optimization
- Execution planning
- Result formatting

## Design Principles

### Safety First

Rust's type system and ownership model ensure:

- No null pointer dereferencing
- No data races
- No buffer overflows
- Memory safety without GC

```rust
// Safe by design - compiler enforces correctness
let storage = Storage::new();
// Ownership is clear, no hidden state
```

### Performance

Zero-cost abstractions mean high-level code compiles to efficient machine code:

```rust
// This high-level code...
let result = engine.execute("SELECT * FROM users")?;

// ...compiles to efficient, optimized machine code
```

### Simplicity

Clean, intuitive APIs:

```rust
use synapse_db::{Storage, QueryEngine};

let storage = Storage::new();
let engine = QueryEngine::new();
```

## Common Patterns

### Basic Usage

```rust
use synapse_db::{storage::Storage, query::QueryEngine};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let storage = Storage::new();
    let engine = QueryEngine::new();

    // Your application logic

    Ok(())
}
```

### Error Handling

```rust
fn process_query(engine: &QueryEngine, sql: &str) -> Result<(), String> {
    engine.execute(sql)
}

match process_query(&engine, "SELECT 1") {
    Ok(_) => println!("Success"),
    Err(e) => eprintln!("Error: {}", e),
}
```

### Resource Management

Rust's RAII means resources are automatically cleaned up:

```rust
{
    let storage = Storage::new();
    // Use storage
} // Storage automatically dropped here
```

## Performance Characteristics

| Operation | Time Complexity | Notes |
|-----------|----------------|-------|
| Storage creation | O(1) | Constant time |
| Query engine creation | O(1) | Constant time |
| Query execution | Varies | Depends on query complexity |

## Current Limitations

As Synapse DB is in early development, current limitations include:

- No persistent storage implementation yet
- Query execution returns placeholder errors
- No transaction support
- No concurrent access handling
- Limited SQL syntax support

These are being actively addressed. Check the [Changelog](../changelog.md) for updates.

## Next Steps

Dive deeper into specific components:

- [Storage Layer](./storage.md) - Data persistence
- [Query Engine](./query-engine.md) - Query processing
- [Examples](./examples.md) - Code examples
- [Best Practices](./best-practices.md) - Usage recommendations

## Getting Help

- [FAQ](../faq.md) - Common questions
- [API Reference](../api/overview.md) - Detailed API docs
- [GitHub Issues](https://github.com/ajber/synapse-db/issues) - Report bugs
