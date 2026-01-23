# Quick Start

Get up and running with Synapse DB in minutes.

## Your First Synapse DB Application

### 1. Create a New Project

```bash
cargo new my-db-app
cd my-db-app
```

### 2. Add Synapse DB Dependency

Edit `Cargo.toml`:

```toml
[package]
name = "my-db-app"
version = "0.1.0"
edition = "2021"

[dependencies]
synapse-db = "0.1"
```

### 3. Write Your First Program

Edit `src/main.rs`:

```rust
use synapse_db::{storage::Storage, query::QueryEngine};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Initializing Synapse DB...");

    // Create storage instance
    let storage = Storage::new();
    println!("✓ Storage initialized: {:?}", storage);

    // Create query engine
    let engine = QueryEngine::new();
    println!("✓ Query engine initialized: {:?}", engine);

    println!("\nSynapse DB is ready!");

    Ok(())
}
```

### 4. Run Your Application

```bash
cargo run
```

Expected output:

```
Initializing Synapse DB...
✓ Storage initialized: Storage
✓ Query engine initialized: QueryEngine
Synapse DB is ready!
```

## Basic Examples

### Storage Operations

```rust
use synapse_db::storage::Storage;

fn main() {
    // Create storage with default configuration
    let storage = Storage::new();

    // Storage is ready for operations
    println!("Storage created successfully");
}
```

### Query Execution

```rust
use synapse_db::query::QueryEngine;

fn main() {
    let engine = QueryEngine::new();

    // Attempt to execute a query
    match engine.execute("SELECT 1") {
        Ok(_) => println!("Query executed successfully"),
        Err(e) => println!("Query failed: {}", e),
    }
}
```

> **Note**: Query execution currently returns an error as the feature is under development.

### Complete Application

```rust
use synapse_db::{storage::Storage, query::QueryEngine};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize components
    let _storage = Storage::new();
    let engine = QueryEngine::new();

    // Process queries
    process_query(&engine, "SELECT * FROM users")?;

    Ok(())
}

fn process_query(engine: &QueryEngine, sql: &str) -> Result<(), String> {
    println!("Executing: {}", sql);
    engine.execute(sql)
}
```

## Common Patterns

### Error Handling

```rust
use synapse_db::query::QueryEngine;

fn run_queries() -> Result<(), String> {
    let engine = QueryEngine::new();

    // Early return on error
    engine.execute("CREATE TABLE users")?;
    engine.execute("INSERT INTO users VALUES (1, 'Alice')")?;

    Ok(())
}

fn main() {
    match run_queries() {
        Ok(_) => println!("All queries succeeded"),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### Using the Default Trait

```rust
use synapse_db::{storage::Storage, query::QueryEngine};

fn main() {
    // Using Default trait implementation
    let storage = Storage::default();
    let engine = QueryEngine::default();

    println!("Initialized with defaults");
}
```

### Resource Management

```rust
use synapse_db::storage::Storage;

fn main() {
    {
        let storage = Storage::new();
        // Use storage
        println!("Storage in use");
    } // Storage automatically cleaned up here

    println!("Storage dropped");
}
```

## Testing Your Code

### Writing Tests

Create `src/lib.rs`:

```rust
#[cfg(test)]
mod tests {
    use synapse_db::{storage::Storage, query::QueryEngine};

    #[test]
    fn test_storage_creation() {
        let storage = Storage::new();
        // Add your assertions
    }

    #[test]
    fn test_query_engine() {
        let engine = QueryEngine::new();
        let result = engine.execute("SELECT 1");
        // Currently returns error (not implemented)
        assert!(result.is_err());
    }
}
```

Run tests:

```bash
cargo test
```

### Integration Tests

Create `tests/integration_test.rs`:

```rust
use synapse_db::{storage::Storage, query::QueryEngine};

#[test]
fn test_end_to_end() {
    let _storage = Storage::new();
    let engine = QueryEngine::new();

    // Test your workflow
    let result = engine.execute("SELECT 1");
    assert!(result.is_err()); // Currently not implemented
}
```

## Development Workflow

### Check Code Quality

```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Run all checks
cargo check
```

### Build Variants

```bash
# Debug build (faster compilation)
cargo build

# Release build (optimized)
cargo build --release

# Check without building
cargo check
```

### Running Examples

If you create example files in `examples/`:

```bash
cargo run --example my_example
```

## Next Steps

Now that you have Synapse DB running:

- **[Configuration](./configuration.md)** - Learn about configuration options
- **[Storage Layer](../guide/storage.md)** - Deep dive into storage
- **[Query Engine](../guide/query-engine.md)** - Understanding queries
- **[Examples](../guide/examples.md)** - More advanced examples
- **[API Reference](../api/overview.md)** - Detailed API documentation

## Common Tasks

### Adding More Dependencies

Edit `Cargo.toml`:

```toml
[dependencies]
synapse-db = "0.1"
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
```

### Setting Up Logging

```toml
[dependencies]
synapse-db = "0.1"
env_logger = "0.11"
log = "0.4"
```

```rust
use log::info;

fn main() {
    env_logger::init();
    info!("Starting Synapse DB application");

    let storage = synapse_db::storage::Storage::new();
    info!("Storage initialized");
}
```

### Benchmarking

Create `benches/my_benchmark.rs`:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use synapse_db::storage::Storage;

fn benchmark_storage_creation(c: &mut Criterion) {
    c.bench_function("storage_new", |b| {
        b.iter(|| Storage::new());
    });
}

criterion_group!(benches, benchmark_storage_creation);
criterion_main!(benches);
```

Run benchmarks:

```bash
cargo bench
```

## Troubleshooting

### Compilation Errors

Update dependencies:

```bash
cargo update
```

Clean build artifacts:

```bash
cargo clean
cargo build
```

### Runtime Issues

Check Rust version:

```bash
rustc --version
```

Ensure you're using Rust 1.70.0 or later.

## Getting Help

- **[FAQ](../faq.md)** - Common questions
- **[GitHub Issues](https://github.com/ajber/synapse-db/issues)** - Report bugs
- **[Discussions](https://github.com/ajber/synapse-db/discussions)** - Ask questions

---

Continue to the [User Guide](../guide/overview.md) for in-depth information!
