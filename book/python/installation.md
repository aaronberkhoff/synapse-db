# Installation

## From PyPI (Recommended)

Once published, install Synapse DB from PyPI:

```bash
pip install synapse-db
```

## From Source

To build from source, you'll need:

- **Rust**: 1.70 or later ([rustup.rs](https://rustup.rs))
- **Python**: 3.9 or later
- **Maturin**: Build tool for Rust Python extensions

### Step 1: Clone the Repository

```bash
git clone https://github.com/yourusername/synapse-db.git
cd synapse-db
```

### Step 2: Create a Virtual Environment (Recommended)

```bash
python -m venv .venv
source .venv/bin/activate  # On Windows: .venv\Scripts\activate
```

### Step 3: Install Maturin

```bash
pip install maturin
```

### Step 4: Build and Install

For development (editable install):

```bash
maturin develop
```

For a release build:

```bash
maturin develop --release
```

### Step 5: Verify Installation

```python
>>> import synapse_db
>>> synapse_db.version()
'0.1.0'
```

## Building Wheels

To build distributable wheel files:

```bash
# Build for current platform
maturin build --release

# Build for all supported Python versions
maturin build --release --find-interpreter
```

Wheels will be placed in `target/wheels/`.

## Development Dependencies

For development and testing:

```bash
pip install -e ".[dev]"
```

This installs:
- `pytest` - Testing framework
- `pytest-cov` - Coverage reporting
- `mypy` - Type checking

## Generating Type Stubs

Type stubs (`.pyi` files) are automatically generated from the Rust code using `pyo3-stub-gen`. To regenerate stubs after modifying the Rust bindings:

```bash
./scripts/generate_stubs.sh
```

Or manually:

```bash
cargo run --bin stub_gen
```

The stubs are generated in `python/synapse_db/synapse_db.pyi` and include:

- Type annotations for all classes and functions
- Docstrings extracted from Rust doc comments
- Parameter and return type information

## Platform Support

Synapse DB Python bindings support:

| Platform | Architecture | Status |
|----------|-------------|--------|
| Linux | x86_64 | Supported |
| Linux | aarch64 | Supported |
| macOS | x86_64 | Supported |
| macOS | aarch64 (Apple Silicon) | Supported |
| Windows | x86_64 | Supported |

## Troubleshooting

### Rust Not Found

If you see "cargo not found", ensure Rust is installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### Python Version Mismatch

Ensure you're using Python 3.9+:

```bash
python --version
```

### Build Failures on Linux

You may need development headers:

```bash
# Ubuntu/Debian
sudo apt install python3-dev

# Fedora
sudo dnf install python3-devel
```
