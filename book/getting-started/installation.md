# Installation

This guide will help you install and set up Synapse DB on your system.

## Prerequisites

- Rust 1.70.0 or later
- Cargo (comes with Rust)

## Installing Rust

If you don't have Rust installed, use [rustup](https://rustup.rs/):

### Linux / macOS

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Windows

Download and run [rustup-init.exe](https://win.rustup.rs/)

### Verify Installation

```bash
rustc --version
cargo --version
```

You should see output like:
```
rustc 1.92.0 (ded5c06cf 2025-12-08)
cargo 1.92.0 (344c4567c 2025-10-21)
```

## Installing Synapse DB

There are several ways to install Synapse DB:

### Option 1: From crates.io (Recommended)

Add Synapse DB to your project's `Cargo.toml`:

```toml
[dependencies]
synapse-db = "0.1"
```

Then run:

```bash
cargo build
```

Cargo will automatically download and compile Synapse DB and its dependencies.

### Option 2: From Source

Clone the repository and build:

```bash
git clone https://github.com/ajber/synapse-db.git
cd synapse-db
cargo build --release
```

The compiled binary will be at `target/release/synapse-db`.

### Option 3: Pre-built Binaries

Download pre-built binaries from the [releases page](https://github.com/ajber/synapse-db/releases):

#### Linux

```bash
# Download and extract
wget https://github.com/ajber/synapse-db/releases/latest/download/synapse-db-linux-amd64.tar.gz
tar xzf synapse-db-linux-amd64.tar.gz

# Move to PATH
sudo mv synapse-db /usr/local/bin/

# Verify
synapse-db --version
```

#### macOS

```bash
# Intel Macs
curl -L https://github.com/ajber/synapse-db/releases/latest/download/synapse-db-macos-amd64.tar.gz -o synapse-db.tar.gz

# Apple Silicon (M1/M2)
curl -L https://github.com/ajber/synapse-db/releases/latest/download/synapse-db-macos-arm64.tar.gz -o synapse-db.tar.gz

# Extract and install
tar xzf synapse-db.tar.gz
sudo mv synapse-db /usr/local/bin/

# Verify
synapse-db --version
```

#### Windows

1. Download `synapse-db-windows-amd64.zip` from the [releases page](https://github.com/ajber/synapse-db/releases)
2. Extract the archive
3. Add the directory to your PATH
4. Open a new terminal and run `synapse-db --version`

## Verifying Installation

### As a Binary

Run the binary to verify it works:

```bash
synapse-db --version
```

### As a Library

Create a test project:

```bash
cargo new synapse-test
cd synapse-test
```

Add to `Cargo.toml`:

```toml
[dependencies]
synapse-db = "0.1"
```

Edit `src/main.rs`:

```rust
use synapse_db::{storage::Storage, query::QueryEngine};

fn main() {
    let storage = Storage::new();
    let engine = QueryEngine::new();
    println!("✓ Synapse DB initialized successfully!");
}
```

Run it:

```bash
cargo run
```

You should see: `✓ Synapse DB initialized successfully!`

## Platform-Specific Notes

### Linux

- Requires glibc 2.17 or later
- For musl-based systems, use the `linux-musl` binary

### macOS

- Minimum version: macOS 10.12 Sierra
- Intel and Apple Silicon binaries available

### Windows

- Minimum version: Windows 7 SP1
- Requires Visual C++ Redistributable

## Troubleshooting

### Rust Not Found

If `cargo` is not found after installation:

```bash
# Load Cargo environment
source $HOME/.cargo/env

# Or add to your shell profile (~/.bashrc, ~/.zshrc)
echo 'source $HOME/.cargo/env' >> ~/.bashrc
```

### Build Errors

Ensure you have the latest stable Rust:

```bash
rustup update stable
```

### Linker Errors on Linux

Install build essentials:

```bash
# Debian/Ubuntu
sudo apt-get update
sudo apt-get install build-essential

# Fedora/RHEL
sudo dnf install gcc

# Arch
sudo pacman -S base-devel
```

### Permission Denied (macOS)

If you get "permission denied" when running the binary:

```bash
chmod +x synapse-db
```

### Windows Antivirus

Some antivirus software may flag Rust binaries. Add an exception for the Synapse DB binary.

## Development Setup

For contributing to Synapse DB, see the [Contributing Guide](../development/contributing.md) for additional setup requirements.

## Next Steps

- **[Quick Start](./quick-start.md)** - Build your first application
- **[Configuration](./configuration.md)** - Configure Synapse DB
- **[Examples](../guide/examples.md)** - See example code

## Getting Help

- Check the [FAQ](../faq.md)
- Search [existing issues](https://github.com/ajber/synapse-db/issues)
- Open a [new issue](https://github.com/ajber/synapse-db/issues/new)
