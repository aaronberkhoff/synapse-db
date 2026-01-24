#!/bin/bash
# Generate Python type stubs from Rust code
#
# This script builds and runs the stub generator to create .pyi files
# that provide type hints for the Python bindings.
#
# Usage: ./scripts/generate_stubs.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

echo "Building stub generator..."
cargo build --bin stub_gen --features python

echo "Generating Python type stubs..."
cargo run --bin stub_gen --features python

# Move the generated stub file to the correct location in the package
if [ -f "$PROJECT_DIR/python/synapse_db.pyi" ]; then
    mv "$PROJECT_DIR/python/synapse_db.pyi" "$PROJECT_DIR/python/synapse_db/synapse_db.pyi"
    echo "Moved stubs to python/synapse_db/synapse_db.pyi"
fi

echo "Done! Stubs generated in python/synapse_db/"
echo ""
echo "Generated files:"
ls -la "$PROJECT_DIR/python/synapse_db/"*.pyi 2>/dev/null || echo "  (no .pyi files found)"
