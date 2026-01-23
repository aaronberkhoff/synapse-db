#!/bin/bash
# Pre-push check script - Run this before pushing to ensure CI will pass

set -e

echo "🔍 Running pre-push checks..."
echo ""

# Color codes
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    if [ $1 -eq 0 ]; then
        echo -e "${GREEN}✓${NC} $2"
    else
        echo -e "${RED}✗${NC} $2"
        exit 1
    fi
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

# 1. Format check
echo "📝 Checking code formatting..."
cargo fmt --all -- --check
print_status $? "Code formatting"

# 2. Clippy
echo ""
echo "🔧 Running clippy..."
cargo clippy --all-targets --all-features -- -D warnings
print_status $? "Clippy lints"

# 3. Tests
echo ""
echo "🧪 Running tests..."
cargo test --all-features
print_status $? "Tests"

# 4. Doc tests
echo ""
echo "📚 Running doc tests..."
cargo test --doc
print_status $? "Doc tests"

# 5. Build
echo ""
echo "🏗️  Building release..."
cargo build --release
print_status $? "Release build"

# 6. Security audit (optional, warn only)
echo ""
echo "🔒 Running security audit..."
if command -v cargo-audit &> /dev/null; then
    if cargo audit; then
        print_status 0 "Security audit"
    else
        print_warning "Security audit found issues (not blocking)"
    fi
else
    print_warning "cargo-audit not installed (run: cargo install cargo-audit)"
fi

# 7. Check for large files
echo ""
echo "📦 Checking for large files..."
large_files=$(find . -type f -size +1M -not -path "*/target/*" -not -path "*/.git/*" | wc -l)
if [ "$large_files" -eq 0 ]; then
    print_status 0 "No large files"
else
    print_warning "Found $large_files large file(s) (>1MB)"
fi

echo ""
echo -e "${GREEN}✅ All checks passed! Safe to push.${NC}"
echo ""
echo "To push your changes:"
echo "  git push origin <branch-name>"
echo ""
echo "To create a release:"
echo "  git tag v0.1.0"
echo "  git push origin v0.1.0"
