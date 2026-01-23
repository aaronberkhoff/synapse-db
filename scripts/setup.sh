#!/bin/bash
# Synapse DB Development Environment Setup
# This script sets up everything you need to start developing

set -e

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Helper functions
print_header() {
    echo -e "\n${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${CYAN}$1${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}\n"
}

print_status() {
    if [ $1 -eq 0 ]; then
        echo -e "${GREEN}✓${NC} $2"
    else
        echo -e "${RED}✗${NC} $2"
        return 1
    fi
}

print_info() {
    echo -e "${BLUE}ℹ${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

# Banner
echo -e "${MAGENTA}"
cat << 'EOF'
   ____                                       ____  ____
  / ___| _   _ _ __   __ _ _ __  ___  ___   |  _ \| __ )
  \___ \| | | | '_ \ / _` | '_ \/ __|/ _ \  | | | |  _ \
   ___) | |_| | | | | (_| | |_) \__ \  __/  | |_| | |_) |
  |____/ \__, |_| |_|\__,_| .__/|___/\___|  |____/|____/
         |___/            |_|

  Development Environment Setup
EOF
echo -e "${NC}"

print_info "This script will set up your development environment for Synapse DB"
echo ""

# Check if running on supported OS
print_header "1/8 Checking System Requirements"

OS="$(uname -s)"
case "${OS}" in
    Linux*)     MACHINE=Linux;;
    Darwin*)    MACHINE=Mac;;
    CYGWIN*)    MACHINE=Cygwin;;
    MINGW*)     MACHINE=MinGw;;
    *)          MACHINE="UNKNOWN:${OS}"
esac

print_status 0 "Operating System: $MACHINE"

# Check for Rust installation
print_header "2/8 Checking Rust Installation"

if command -v rustc &> /dev/null && command -v cargo &> /dev/null; then
    RUST_VERSION=$(rustc --version)
    CARGO_VERSION=$(cargo --version)
    print_status 0 "Rust is already installed"
    print_info "  $RUST_VERSION"
    print_info "  $CARGO_VERSION"
else
    print_warning "Rust is not installed. Installing Rust..."

    if [ "$MACHINE" = "MinGw" ] || [ "$MACHINE" = "Cygwin" ]; then
        print_error "Please install Rust manually from https://rustup.rs/"
        exit 1
    else
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

        # Source cargo environment
        if [ -f "$HOME/.cargo/env" ]; then
            source "$HOME/.cargo/env"
        fi

        print_status 0 "Rust installed successfully"
    fi
fi

# Ensure cargo is in PATH
if ! command -v cargo &> /dev/null; then
    if [ -f "$HOME/.cargo/env" ]; then
        source "$HOME/.cargo/env"
    fi

    if ! command -v cargo &> /dev/null; then
        print_error "Cargo not found in PATH. Please restart your shell or run:"
        echo -e "  ${CYAN}source \$HOME/.cargo/env${NC}"
        exit 1
    fi
fi

# Update Rust
print_header "3/8 Updating Rust Toolchain"

rustup update stable
print_status 0 "Rust toolchain updated"

# Install development tools
print_header "4/8 Installing Development Tools"

echo -e "${BLUE}Installing required Cargo tools...${NC}\n"

# mdBook for documentation
print_info "Installing mdBook..."
if command -v mdbook &> /dev/null; then
    print_status 0 "mdBook already installed ($(mdbook --version))"
else
    cargo install mdbook --locked
    print_status 0 "mdBook installed"
fi

# git-cliff for changelog generation
print_info "Installing git-cliff..."
if command -v git-cliff &> /dev/null; then
    print_status 0 "git-cliff already installed"
else
    cargo install git-cliff --locked
    print_status 0 "git-cliff installed"
fi

# cargo-audit for security audits
print_info "Installing cargo-audit..."
if command -v cargo-audit &> /dev/null; then
    print_status 0 "cargo-audit already installed"
else
    cargo install cargo-audit --locked
    print_status 0 "cargo-audit installed"
fi

# cargo-watch for development (optional but useful)
print_info "Installing cargo-watch (optional)..."
if command -v cargo-watch &> /dev/null; then
    print_status 0 "cargo-watch already installed"
else
    cargo install cargo-watch --locked || {
        print_warning "cargo-watch installation failed (non-critical)"
    }
    print_status 0 "cargo-watch installed"
fi

# Install Git hooks
print_header "5/8 Installing Git Hooks"

if [ -f "./scripts/install-hooks.sh" ]; then
    ./scripts/install-hooks.sh
    print_status 0 "Git hooks installed"
else
    print_warning "install-hooks.sh not found, skipping"
fi

# Build the project
print_header "6/8 Building Project"

print_info "Building Synapse DB..."
cargo build
print_status 0 "Debug build completed"

print_info "Building release version..."
cargo build --release
print_status 0 "Release build completed"

# Run tests
print_header "7/8 Running Tests"

print_info "Running test suite..."
cargo test
print_status 0 "All tests passed"

# Build documentation
print_header "8/8 Building Documentation"

print_info "Building mdBook documentation..."
mdbook build
print_status 0 "mdBook documentation built"

print_info "Building Rust API documentation..."
cargo doc --no-deps --all-features
print_status 0 "API documentation built"

print_info "Generating changelog..."
if [ -f "./scripts/generate-changelog.sh" ]; then
    ./scripts/generate-changelog.sh
    print_status 0 "Changelog generated"
else
    print_warning "generate-changelog.sh not found, skipping"
fi

# Final summary
echo -e "\n${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}✓ Setup Complete!${NC}"
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}\n"

echo -e "${CYAN}Next Steps:${NC}\n"

echo -e "${YELLOW}Development Commands:${NC}"
echo -e "  ${GREEN}cargo run${NC}          - Run the binary"
echo -e "  ${GREEN}cargo test${NC}         - Run tests"
echo -e "  ${GREEN}cargo build --release${NC} - Build optimized binary"
echo -e "  ${GREEN}cargo fmt${NC}          - Format code"
echo -e "  ${GREEN}cargo clippy${NC}       - Run linter"
echo ""

echo -e "${YELLOW}Documentation:${NC}"
echo -e "  ${GREEN}mdbook serve${NC}       - Serve docs with live reload"
echo -e "  ${GREEN}cargo doc --open${NC}   - Open API documentation"
echo ""

echo -e "${YELLOW}Pre-commit Checks:${NC}"
echo -e "  ${GREEN}./scripts/pre-push-check.sh${NC} - Run all checks"
echo ""

echo -e "${YELLOW}Git Hooks:${NC}"
echo -e "  Pre-commit hooks are active and will:"
echo -e "    • Auto-format code with ${GREEN}rustfmt${NC}"
echo -e "    • Auto-fix linting issues with ${GREEN}clippy${NC}"
echo -e "    • Validate commit messages (conventional commits)"
echo ""

echo -e "${YELLOW}Development Workflow:${NC}"
echo -e "  1. Create a branch: ${GREEN}git checkout -b feature/my-feature${NC}"
echo -e "  2. Make changes and commit: ${GREEN}git commit -m \"feat: add my feature\"${NC}"
echo -e "  3. Run checks: ${GREEN}./scripts/pre-push-check.sh${NC}"
echo -e "  4. Push changes: ${GREEN}git push origin feature/my-feature${NC}"
echo ""

echo -e "${YELLOW}Useful Tools:${NC}"
echo -e "  ${GREEN}cargo watch -x test${NC}     - Auto-run tests on file changes"
echo -e "  ${GREEN}cargo watch -x run${NC}      - Auto-run on file changes"
echo -e "  ${GREEN}cargo audit${NC}             - Check for security vulnerabilities"
echo ""

echo -e "${YELLOW}Documentation:${NC}"
echo -e "  • User Guide: ${CYAN}book/README.md${NC}"
echo -e "  • Contributing: ${CYAN}CONTRIBUTING.md${NC}"
echo -e "  • CI/CD Info: ${CYAN}.github/WORKFLOWS.md${NC}"
echo -e "  • Hooks Info: ${CYAN}.github/HOOKS.md${NC}"
echo ""

echo -e "${GREEN}Happy coding! 🚀${NC}\n"

# Check if PATH includes cargo
if [[ ":$PATH:" != *":$HOME/.cargo/bin:"* ]]; then
    echo -e "${YELLOW}⚠  Note: Add Cargo to your PATH by adding this to your shell profile:${NC}"
    echo -e "   ${CYAN}source \$HOME/.cargo/env${NC}"
    echo ""
fi
