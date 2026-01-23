# Development Scripts

This directory contains automation scripts for Synapse DB development.

## Available Scripts

### 🚀 setup.sh

**Complete development environment setup**

```bash
./scripts/setup.sh
```

**What it does:**
- Installs/updates Rust and Cargo
- Installs development tools (mdBook, git-cliff, cargo-audit, cargo-watch)
- Sets up Git hooks
- Builds the project (debug and release)
- Runs tests
- Builds documentation

**When to use:** First-time setup or setting up a new machine

**Time:** ~10-15 minutes

---

### 🎣 install-hooks.sh

**Install Git hooks for code quality**

```bash
./scripts/install-hooks.sh
```

**What it does:**
- Copies Git hooks from `hooks/` to `.git/hooks/`
- Makes hooks executable
- Enables pre-commit formatting and linting
- Enables commit message validation

**When to use:**
- After cloning the repository
- After pulling changes to hooks
- If hooks were accidentally removed

**Time:** < 1 second

---

### ✅ pre-push-check.sh

**Run all CI checks locally before pushing**

```bash
./scripts/pre-push-check.sh
```

**What it does:**
- Checks code formatting (`cargo fmt`)
- Runs Clippy linter (`cargo clippy`)
- Runs all tests (`cargo test`)
- Runs doc tests
- Builds release binary
- Runs security audit (if `cargo-audit` installed)
- Checks for large files

**When to use:**
- Before pushing to remote
- Before creating a pull request
- To verify all CI checks will pass

**Time:** ~2-10 seconds (depends on cache)

---

### 📝 generate-changelog.sh

**Generate changelog from git commits**

```bash
./scripts/generate-changelog.sh
```

**What it does:**
- Installs `git-cliff` if not present
- Generates `CHANGELOG.md` from git history
- Copies changelog to `book/changelog.md`
- Uses conventional commits to categorize changes

**When to use:**
- Before creating a release
- When you want to update the changelog
- After significant development work

**Time:** < 5 seconds

**Requirements:** Commits must follow [Conventional Commits](https://www.conventionalcommits.org/)

---

## Script Usage Patterns

### New Developer Onboarding

```bash
# 1. Clone repository
git clone https://github.com/aaronberkhoff/synapse-db.git
cd synapse-db

# 2. Run setup (only needed once)
./scripts/setup.sh

# 3. Start developing!
cargo run
```

### Daily Development Workflow

```bash
# 1. Make changes to code

# 2. Commit (hooks run automatically)
git commit -m "feat: add new feature"

# 3. Before pushing, run checks
./scripts/pre-push-check.sh

# 4. Push if all checks pass
git push
```

### Release Workflow

```bash
# 1. Update version in Cargo.toml

# 2. Generate changelog
./scripts/generate-changelog.sh

# 3. Review and commit changelog
git add CHANGELOG.md book/changelog.md
git commit -m "docs: update changelog for v0.2.0"

# 4. Create and push tag
git tag v0.2.0
git push origin v0.2.0

# CI will automatically build and release
```

### Updating Hooks

```bash
# After pulling changes that updated hooks
./scripts/install-hooks.sh
```

## Creating New Scripts

When adding new scripts to this directory:

1. **Make it executable:**
   ```bash
   chmod +x scripts/my-script.sh
   ```

2. **Add a shebang:**
   ```bash
   #!/bin/bash
   ```

3. **Set error handling:**
   ```bash
   set -e  # Exit on error
   ```

4. **Add documentation:**
   - Add header comment describing what the script does
   - Update this README
   - Add usage examples

5. **Use consistent style:**
   - Color codes for output
   - Status indicators (✓, ✗, ⚠)
   - Clear error messages

## Script Template

```bash
#!/bin/bash
# Description: What this script does

set -e

# Color codes
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

# Helper functions
print_status() {
    if [ $1 -eq 0 ]; then
        echo -e "${GREEN}✓${NC} $2"
    else
        echo -e "${RED}✗${NC} $2"
        exit 1
    fi
}

# Main script logic
echo "Starting my script..."

# Do something
result=$?
print_status $result "Task completed"
```

## Troubleshooting

### Script won't run

**Error:** `permission denied`

**Solution:**
```bash
chmod +x scripts/script-name.sh
```

### Script not found

**Error:** `scripts/script-name.sh: No such file or directory`

**Solution:**
Ensure you're in the repository root:
```bash
cd /path/to/synapse-db
./scripts/script-name.sh
```

### Hooks not working

**Error:** Git hooks not running

**Solution:**
```bash
./scripts/install-hooks.sh
```

### Pre-push check fails

**Error:** Check failures

**Solution:**
```bash
# Format code
cargo fmt

# Fix clippy issues
cargo clippy --fix

# Run again
./scripts/pre-push-check.sh
```

## Environment Variables

Scripts may respect these environment variables:

- `CARGO_HOME` - Cargo installation directory (default: `~/.cargo`)
- `RUSTUP_HOME` - Rustup installation directory

## Dependencies

Scripts require:

- **Bash** - All scripts are Bash scripts
- **Git** - For version control operations
- **Cargo** - For Rust operations (installed by setup.sh)

Optional dependencies (installed by setup.sh):

- **mdBook** - Documentation building
- **git-cliff** - Changelog generation
- **cargo-audit** - Security scanning

## Testing Scripts

To test scripts without running them:

```bash
# Check syntax
bash -n scripts/script-name.sh

# Dry run (if supported)
./scripts/script-name.sh --dry-run
```

## CI/CD Integration

These scripts are also used in GitHub Actions workflows:

- `setup.sh` - Not used in CI (CI has its own setup)
- `install-hooks.sh` - Not used in CI (no git operations)
- `pre-push-check.sh` - Similar checks run in CI
- `generate-changelog.sh` - Used in docs workflow

See [.github/workflows/](.github/workflows/) for CI configurations.

## Best Practices

1. **Always use `set -e`** - Exit on first error
2. **Use color codes** - Make output readable
3. **Provide status messages** - Keep user informed
4. **Handle errors gracefully** - Don't leave system in bad state
5. **Be idempotent** - Script can run multiple times safely
6. **Check prerequisites** - Verify required tools exist
7. **Use descriptive names** - Script name should indicate purpose
8. **Document everything** - Comments and this README

## Contributing

When modifying scripts:

1. Test thoroughly on multiple platforms (Linux, macOS, WSL)
2. Update this README
3. Follow existing code style
4. Use conventional commit messages
5. Test in CI (if applicable)

## Getting Help

- **Script Questions**: Open an issue
- **Bug Reports**: Include script name and error output
- **Feature Requests**: Describe what automation you need

---

**Last Updated:** 2026-01-22
