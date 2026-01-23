#!/bin/bash
# Install git hooks for Synapse DB

set -e

# Color codes
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${BLUE}Installing Git hooks...${NC}\n"

# Get the git root directory
GIT_DIR=$(git rev-parse --git-dir)
HOOKS_DIR="$GIT_DIR/hooks"
PROJECT_HOOKS_DIR="$(git rev-parse --show-toplevel)/hooks"

# Ensure hooks directory exists
mkdir -p "$HOOKS_DIR"

# Install pre-commit hook
if [ -f "$PROJECT_HOOKS_DIR/pre-commit" ]; then
    cp "$PROJECT_HOOKS_DIR/pre-commit" "$HOOKS_DIR/pre-commit"
    chmod +x "$HOOKS_DIR/pre-commit"
    echo -e "${GREEN}✓${NC} Installed pre-commit hook"
else
    echo -e "${YELLOW}⚠${NC} pre-commit hook not found in hooks/ directory"
fi

# Install commit-msg hook if it exists
if [ -f "$PROJECT_HOOKS_DIR/commit-msg" ]; then
    cp "$PROJECT_HOOKS_DIR/commit-msg" "$HOOKS_DIR/commit-msg"
    chmod +x "$HOOKS_DIR/commit-msg"
    echo -e "${GREEN}✓${NC} Installed commit-msg hook"
fi

# Install prepare-commit-msg hook if it exists
if [ -f "$PROJECT_HOOKS_DIR/prepare-commit-msg" ]; then
    cp "$PROJECT_HOOKS_DIR/prepare-commit-msg" "$HOOKS_DIR/prepare-commit-msg"
    chmod +x "$HOOKS_DIR/prepare-commit-msg"
    echo -e "${GREEN}✓${NC} Installed prepare-commit-msg hook"
fi

echo ""
echo -e "${GREEN}✅ Git hooks installed successfully!${NC}"
echo ""
echo "Installed hooks:"
echo "  • pre-commit: Auto-formats and lints code before commit"
echo ""
echo "To bypass hooks temporarily (not recommended):"
echo "  git commit --no-verify"
echo ""
