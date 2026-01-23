# Configuration

Learn how to configure Synapse DB for your use case.

> **Note**: Configuration options are currently under development. This page documents the planned configuration system and will be updated as features are implemented.

## Current State

Synapse DB currently uses default configurations for all components. Custom configuration support is planned for future releases.

### Basic Usage

```rust
use synapse_db::{storage::Storage, query::QueryEngine};

fn main() {
    // Default configuration
    let storage = Storage::new();
    let engine = QueryEngine::new();
}
```

## Planned Configuration

### Configuration File (Planned)

Future versions will support TOML configuration files:

```toml
# synapse-db.toml

[storage]
path = "/var/lib/synapse-db/data"
cache_size_mb = 1024
max_file_size_mb = 100
compression = true

[query]
max_concurrent_queries = 100
query_timeout_secs = 30
enable_optimization = true

[logging]
level = "info"
output = "stdout"
file = "/var/log/synapse-db/app.log"

[performance]
thread_pool_size = 4
buffer_size_kb = 8
enable_profiling = false
```

Loading configuration:

```rust
use synapse_db::config::Config;

fn main() {
    let config = Config::from_file("synapse-db.toml")?;
    let storage = Storage::with_config(config.storage)?;
    let engine = QueryEngine::with_config(config.query)?;
}
```

### Environment Variables (Planned)

Configuration via environment variables:

```bash
# Storage configuration
export SYNAPSE_DB_STORAGE_PATH="/var/lib/synapse-db/data"
export SYNAPSE_DB_CACHE_SIZE_MB=1024

# Query configuration
export SYNAPSE_DB_MAX_CONCURRENT=100
export SYNAPSE_DB_QUERY_TIMEOUT=30

# Logging
export SYNAPSE_DB_LOG_LEVEL=debug
export SYNAPSE_DB_LOG_FILE="/var/log/synapse-db/app.log"

# Performance
export SYNAPSE_DB_THREADS=4
```

Usage:

```rust
use synapse_db::config::Config;

fn main() {
    // Loads from environment variables
    let config = Config::from_env()?;
    let storage = Storage::with_config(config.storage)?;
}
```

### Programmatic Configuration (Planned)

Builder pattern for runtime configuration:

```rust
use synapse_db::{Storage, QueryEngine};
use synapse_db::config::{StorageConfig, QueryConfig};

fn main() {
    // Configure storage
    let storage_config = StorageConfig::new()
        .path("/data/synapse-db")
        .cache_size_mb(2048)
        .compression(true);

    let storage = Storage::with_config(storage_config)?;

    // Configure query engine
    let query_config = QueryConfig::new()
        .max_concurrent(200)
        .timeout_secs(60)
        .enable_optimization(true);

    let engine = QueryEngine::with_config(query_config)?;
}
```

## Configuration Options Reference

### Storage Configuration

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `path` | String | `"./data"` | Data directory path |
| `cache_size_mb` | u64 | `512` | Cache size in MB |
| `max_file_size_mb` | u64 | `100` | Maximum file size |
| `compression` | bool | `false` | Enable compression |
| `sync_on_write` | bool | `true` | Sync to disk on write |

### Query Configuration

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `max_concurrent_queries` | u32 | `100` | Max concurrent queries |
| `query_timeout_secs` | u64 | `30` | Query timeout in seconds |
| `enable_optimization` | bool | `true` | Enable query optimization |
| `max_query_size_kb` | u64 | `1024` | Max query size in KB |

### Logging Configuration

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `level` | String | `"info"` | Log level (debug, info, warn, error) |
| `output` | String | `"stdout"` | Output destination |
| `file` | String | None | Log file path (optional) |
| `format` | String | `"json"` | Log format (json, text) |

### Performance Configuration

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `thread_pool_size` | u32 | CPU count | Worker thread count |
| `buffer_size_kb` | u32 | `8` | I/O buffer size in KB |
| `enable_profiling` | bool | `false` | Enable performance profiling |
| `metrics_port` | u16 | None | Metrics server port (optional) |

## Configuration Validation

The configuration system will validate settings:

```rust
use synapse_db::config::Config;

fn main() {
    match Config::from_file("synapse-db.toml") {
        Ok(config) => {
            // Configuration is valid
            println!("Configuration loaded successfully");
        }
        Err(e) => {
            eprintln!("Invalid configuration: {}", e);
            std::process::exit(1);
        }
    }
}
```

## Best Practices

### Development Configuration

```toml
# dev-config.toml
[storage]
path = "./dev-data"
cache_size_mb = 128

[query]
query_timeout_secs = 5

[logging]
level = "debug"
output = "stdout"
```

### Production Configuration

```toml
# prod-config.toml
[storage]
path = "/var/lib/synapse-db/data"
cache_size_mb = 4096
compression = true
sync_on_write = true

[query]
max_concurrent_queries = 500
query_timeout_secs = 60
enable_optimization = true

[logging]
level = "warn"
file = "/var/log/synapse-db/app.log"
format = "json"

[performance]
thread_pool_size = 8
enable_profiling = true
metrics_port = 9090
```

### Testing Configuration

```toml
# test-config.toml
[storage]
path = "./test-data"
cache_size_mb = 64

[query]
query_timeout_secs = 1

[logging]
level = "error"
```

## Configuration Priority

When multiple configuration sources are available, they are applied in this order (highest to lowest priority):

1. Programmatic configuration (builder pattern)
2. Environment variables
3. Configuration file
4. Default values

Example:

```rust
// Default: cache_size_mb = 512
// File: cache_size_mb = 1024
// Env: SYNAPSE_DB_CACHE_SIZE_MB=2048
// Result: 2048 (environment variable wins)
```

## Dynamic Configuration (Planned)

Reload configuration without restart:

```rust
use synapse_db::config::Config;

let mut config = Config::from_file("synapse-db.toml")?;

// Watch for configuration file changes
config.watch_for_changes()?;

// Or reload manually
config.reload()?;
```

## Configuration Examples

### Minimal Configuration

```toml
[storage]
path = "./data"

[logging]
level = "info"
```

### High-Performance Configuration

```toml
[storage]
path = "/fast-ssd/synapse-db"
cache_size_mb = 8192
compression = false

[query]
max_concurrent_queries = 1000

[performance]
thread_pool_size = 16
buffer_size_kb = 64
```

### Low-Memory Configuration

```toml
[storage]
cache_size_mb = 128

[query]
max_concurrent_queries = 10

[performance]
thread_pool_size = 2
```

## Next Steps

- **[User Guide](../guide/overview.md)** - Learn about features
- **[Storage Layer](../guide/storage.md)** - Deep dive into storage
- **[Query Engine](../guide/query-engine.md)** - Understanding queries
- **[Examples](../guide/examples.md)** - See configuration examples

## See Also

- [Architecture](../development/architecture.md) - System design
- [Best Practices](../guide/best-practices.md) - Usage recommendations
