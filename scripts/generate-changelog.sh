#!/bin/bash
# Generate changelog using git-cliff

set -e

# Color codes
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${BLUE}📝 Generating changelog...${NC}\n"

# Check if git-cliff is installed
if ! command -v git-cliff &> /dev/null; then
    echo -e "${YELLOW}git-cliff not found. Installing...${NC}"
    cargo install git-cliff
fi

# Generate changelog
git-cliff --output CHANGELOG.md

# Copy to book
cp CHANGELOG.md book/changelog.md

echo -e "\n${GREEN}✓ Changelog generated successfully!${NC}"
echo -e "  - CHANGELOG.md (root)"
echo -e "  - book/changelog.md (documentation)"
echo ""
echo "To preview changes:"
echo "  git diff CHANGELOG.md"
echo ""
echo "To commit:"
echo "  git add CHANGELOG.md book/changelog.md"
echo "  git commit -m 'docs: update changelog'"
