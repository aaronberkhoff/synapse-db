# Synapse DB

[![CI](https://github.com/aaronberkhoff/synapse-db/workflows/CI/badge.svg)](https://github.com/aaronberkhoff/synapse-db/actions/workflows/ci.yml)
[![Release](https://github.com/aaronberkhoff/synapse-db/workflows/Release/badge.svg)](https://github.com/aaronberkhoff/synapse-db/actions/workflows/release.yml)
[![codecov](https://codecov.io/gh/aaronberkhoff/synapse-db/branch/main/graph/badge.svg)](https://codecov.io/gh/aaronberkhoff/synapse-db)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A high-performance database implementation written in Rust.

## Features

- Fast storage layer
- Query processing engine
- Type-safe implementation

## Quick Start

### For New Developers

Run the automated setup script to install everything you need:

```bash
git clone https://github.com/aaronberkhoff/synapse-db.git
cd synapse-db
./scripts/setup.sh
```

This script will:
- ✅ Install/update Rust and Cargo
- ✅ Install development tools (mdBook, git-cliff, cargo-audit)
- ✅ Set up Git hooks for code quality
- ✅ Build the project
- ✅ Run tests
- ✅ Build documentation

📖 **For detailed setup instructions and troubleshooting, see [SETUP.md](SETUP.md)**

### Manual Setup

If you prefer manual setup or the script doesn't work on your system:

1. Install Rust from [rustup.rs](https://rustup.rs/)
2. Clone the repository
3. Run `cargo build` and `cargo test`
4. Install tools: `cargo install mdbook git-cliff cargo-audit`
5. Set up hooks: `./scripts/install-hooks.sh`

## Project Structure

```
synapse-db/
├── src/
│   ├── main.rs       # Binary entry point
│   ├── lib.rs        # Library root
│   ├── storage.rs    # Storage layer implementation
│   └── query.rs      # Query engine
├── Cargo.toml        # Project configuration
└── README.md         # This file
```

## Building

Make sure you have Rust installed. If not, install it from [rustup.rs](https://rustup.rs/).

### Using Make (Recommended)

The project includes a Makefile for convenient development:

```bash
make help          # Show all available commands
make build         # Build the project
make test          # Run tests
make run           # Run the application
make doc           # Build documentation
make fmt           # Format code
make lint          # Run linter
make pre-push      # Run all checks before pushing
```

### Using Cargo Directly

```bash
# Build the project
cargo build

# Build with optimizations
cargo build --release

# Run the binary
cargo run

# Run tests
cargo test
```

See `make help` for all available commands.

## Documentation

Comprehensive documentation is available at [https://aaronberkhoff.github.io/synapse-db](https://aaronberkhoff.github.io/synapse-db)

### Building Documentation Locally

```bash
# Using Make
make doc           # Build all documentation
make serve         # Serve docs with live reload
make doc-open      # Build and open API docs

# Using tools directly
mdbook serve       # User guide with live reload
cargo doc --open   # API documentation
```

Documentation source is in the `book/` directory.

## Development

### Quick Commands

```bash
# Development workflow
make dev           # Start dev environment (watch + docs)
make watch         # Auto-rebuild on changes
make watch-test    # Auto-run tests on changes

# Code quality
make fmt           # Format code
make lint          # Run linter
make test          # Run tests
make pre-push      # Run all checks

# Before committing
make commit        # Format + lint (then commit manually)
```

### Using Cargo Directly

```bash
cargo check        # Quick compile check
cargo fmt          # Format code
cargo clippy       # Run linter
cargo test         # Run tests
```

## CI/CD Pipeline

This project uses GitHub Actions for continuous integration and deployment:

### Automated Checks (on every push/PR)
- **Testing**: Runs on Linux, macOS, and Windows with stable and beta Rust
- **Code Formatting**: Enforces `rustfmt` standards
- **Linting**: Runs `clippy` with strict warnings
- **Code Coverage**: Generates coverage reports via `cargo-tarpaulin`
- **Security Audit**: Checks dependencies for known vulnerabilities
- **Build Artifacts**: Creates release binaries for all platforms

### Scheduled Checks (daily)
- **Outdated Dependencies**: Checks for newer package versions
- **Unused Dependencies**: Detects unnecessary dependencies
- **Nightly Testing**: Tests against Rust nightly
- **MSRV Check**: Validates minimum supported Rust version

### Release Process
When you push a tag matching `v*.*.*` (e.g., `v0.1.0`):
1. Automatically builds binaries for:
   - Linux (glibc and musl)
   - macOS (Intel and ARM)
   - Windows
2. Creates a GitHub release with all artifacts
3. Publishes to crates.io (requires `CARGO_REGISTRY_TOKEN` secret)

**Creating a release:**
```bash
git tag v0.1.0
git push origin v0.1.0
```

### Dependency Management
- **Dependabot**: Automatically creates PRs for dependency updates (weekly)
- Updates both Rust crates and GitHub Actions

## Project Documentation

This repository includes comprehensive documentation:

| Document | Purpose |
|----------|---------|
| [README.md](README.md) | Project overview (this file) |
| [SETUP.md](SETUP.md) | Detailed setup instructions |
| [MAKEFILE.md](MAKEFILE.md) | Make command reference |
| [CONTRIBUTING.md](CONTRIBUTING.md) | Contribution guidelines |
| [CHANGELOG.md](CHANGELOG.md) | Auto-generated changelog |
| [DOCUMENTATION.md](DOCUMENTATION.md) | Documentation system overview |
| [book/](book/) | mdBook user guide source |
| [.github/WORKFLOWS.md](.github/WORKFLOWS.md) | CI/CD workflows guide |
| [.github/HOOKS.md](.github/HOOKS.md) | Git hooks documentation |
| [scripts/README.md](scripts/README.md) | Development scripts guide |

## Contributing

We welcome contributions! Please see:

- [CONTRIBUTING.md](CONTRIBUTING.md) - How to contribute
- [SETUP.md](SETUP.md) - Development environment setup
- [.github/HOOKS.md](.github/HOOKS.md) - Git hooks and code quality
- [.github/WORKFLOWS.md](.github/WORKFLOWS.md) - CI/CD pipeline

## License

MIT