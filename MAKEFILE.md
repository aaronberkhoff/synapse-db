# Makefile Command Reference

The Synapse DB Makefile provides convenient shortcuts for common development tasks.

## Quick Reference

```bash
make help          # Show all available commands
make setup         # Complete development setup
make build         # Build the project
make test          # Run tests
make run           # Run the application
make doc           # Build documentation
```

## All Available Commands

### Help

| Command | Description |
|---------|-------------|
| `make help` | Display all available commands with descriptions |
| `make list-targets` | List all make targets |
| `make info` | Show project information |

### Setup

| Command | Description |
|---------|-------------|
| `make setup` | Run complete development environment setup |
| `make install-hooks` | Install Git hooks only |
| `make install-tools` | Install all development tools |

### Building

| Command | Description |
|---------|-------------|
| `make build` | Build the project (debug mode) |
| `make release` | Build optimized release version |
| `make clean` | Remove all build artifacts |
| `make all` | Run fmt, lint, test, build, and doc |
| `make check` | Quick compile check (no code generation) |

### Testing

| Command | Description |
|---------|-------------|
| `make test` | Run all tests |
| `make test-verbose` | Run tests with verbose output |
| `make bench` | Run benchmarks |
| `make coverage` | Generate test coverage report |

### Code Quality

| Command | Description |
|---------|-------------|
| `make fmt` | Format code with rustfmt |
| `make fmt-check` | Check code formatting (don't modify) |
| `make lint` | Run clippy linter |
| `make lint-fix` | Auto-fix clippy warnings |
| `make pre-push` | Run all pre-push checks |
| `make verify` | Verify code quality (fmt-check + lint + test) |
| `make ci` | Run full CI checks locally |

### Documentation

| Command | Description |
|---------|-------------|
| `make doc` | Build all documentation (mdBook + cargo doc) |
| `make doc-open` | Build and open API documentation |
| `make serve` | Serve documentation with live reload |
| `make changelog` | Generate changelog from git commits |

### Running

| Command | Description |
|---------|-------------|
| `make run` | Run the application (debug mode) |
| `make run-release` | Run the application (release mode) |
| `make watch` | Auto-rebuild and run on file changes |
| `make watch-test` | Auto-run tests on file changes |
| `make example NAME=name` | Run specific example |

### Security & Dependencies

| Command | Description |
|---------|-------------|
| `make audit` | Check for security vulnerabilities |
| `make outdated` | Check for outdated dependencies |
| `make update` | Update dependencies |

### Git Operations

| Command | Description |
|---------|-------------|
| `make commit` | Format and lint (prepare for commit) |
| `make push-check` | Alias for pre-push checks |
| `make status` | Show git status |
| `make new-feature NAME=name` | Create new feature branch |

### Release

| Command | Description |
|---------|-------------|
| `make release-prep` | Prepare for release (update changelog) |
| `make tag VERSION=x.y.z` | Create version tag |

### Development

| Command | Description |
|---------|-------------|
| `make dev` | Start development environment (watch + serve docs) |

## Common Workflows

### First-Time Setup

```bash
make setup
```

This runs the full setup script that:
- Installs Rust and Cargo
- Installs development tools
- Sets up Git hooks
- Builds the project
- Runs tests
- Builds documentation

### Daily Development

```bash
# Start coding
make dev           # Opens docs at localhost:3000, watches for changes

# Or manually
make watch         # Auto-rebuild on changes
make serve         # Serve docs (in another terminal)
```

### Before Committing

```bash
make commit        # Format and lint
git add .
git commit -m "feat: add new feature"
```

The pre-commit hook will automatically run formatting and linting.

### Before Pushing

```bash
make pre-push      # Run all checks

# If all checks pass
git push origin feature-branch
```

### Adding a New Feature

```bash
# Create feature branch
make new-feature NAME=awesome-feature

# Develop your feature
make watch-test    # Auto-run tests as you code

# Before committing
make commit

# Before pushing
make pre-push
```

### Preparing a Release

```bash
# Update changelog
make release-prep

# Review CHANGELOG.md
# Update version in Cargo.toml

# Create tag
make tag VERSION=0.2.0

# Push tag
git push origin v0.2.0
```

### Running Tests

```bash
# All tests
make test

# Verbose output
make test-verbose

# With coverage
make coverage

# Auto-run on changes
make watch-test
```

### Building Documentation

```bash
# Build all docs
make doc

# Serve with live reload
make serve

# Open API docs in browser
make doc-open

# Update changelog
make changelog
```

### Code Quality Checks

```bash
# Format code
make fmt

# Check formatting only
make fmt-check

# Run linter
make lint

# Auto-fix linting issues
make lint-fix

# Run all checks
make verify

# Simulate CI locally
make ci
```

## Advanced Usage

### Parallel Execution

Make can run independent targets in parallel:

```bash
# Run with 4 parallel jobs
make -j4 all

# Run specific targets in parallel
make -j fmt lint &
```

### Dry Run

See what a command would do without executing:

```bash
make -n build
make -n test
```

### Verbose Output

See the actual commands being run:

```bash
make build VERBOSE=1
```

### Custom Examples

Run specific examples:

```bash
make example NAME=basic_usage
make example NAME=advanced_query
```

### Custom Variables

Some targets accept variables:

```bash
# Create feature branch
make new-feature NAME=my-feature

# Create tag
make tag VERSION=1.0.0

# Run specific example
make example NAME=demo
```

## Integration with Other Tools

### VS Code

Add to `.vscode/tasks.json`:

```json
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "make build",
      "type": "shell",
      "command": "make build",
      "group": "build"
    },
    {
      "label": "make test",
      "type": "shell",
      "command": "make test",
      "group": "test"
    }
  ]
}
```

### Pre-commit Hook

The pre-commit hook automatically runs:
- `cargo fmt`
- `cargo clippy --fix`

This ensures code quality before committing.

### CI/CD

GitHub Actions workflows use similar commands but call cargo directly for better control and caching.

## Troubleshooting

### Make not found

**macOS:**
```bash
xcode-select --install
```

**Linux (Debian/Ubuntu):**
```bash
sudo apt-get install build-essential
```

**Windows:**
Use WSL or install make via chocolatey:
```bash
choco install make
```

### Permission denied

```bash
chmod +x scripts/*.sh
```

### Command fails

Run with verbose output:
```bash
make build VERBOSE=1
```

Or run the underlying command directly to see full error:
```bash
cargo build
```

## Customizing the Makefile

### Adding New Targets

1. **Choose a category** (`##@` comment)
2. **Add the target**:
   ```makefile
   my-target: ## Description of target
       @echo "Running my target"
       @cargo build --features my-feature
   ```

3. **Test it**:
   ```bash
   make my-target
   ```

### Modifying Existing Targets

Edit the Makefile and test:

```bash
make -n my-target  # Dry run
make my-target     # Actual run
```

## Best Practices

1. **Use `make help`** - See all available commands
2. **Use `make` over `cargo`** - Consistent interface
3. **Run `make pre-push`** - Before every push
4. **Use `make watch`** - During active development
5. **Use `make ci`** - To simulate CI locally

## Comparison: Make vs. Cargo vs. Scripts

| Task | Make | Cargo | Script |
|------|------|-------|--------|
| Build | `make build` | `cargo build` | N/A |
| Test | `make test` | `cargo test` | N/A |
| Format | `make fmt` | `cargo fmt` | N/A |
| Setup | `make setup` | N/A | `./scripts/setup.sh` |
| Pre-push | `make pre-push` | N/A | `./scripts/pre-push-check.sh` |
| Docs | `make serve` | `mdbook serve` | N/A |

**Use Make when:** You want a simple, memorable command
**Use Cargo directly when:** You need specific flags or options
**Use Scripts when:** You need complex logic or one-time setup

## FAQ

**Q: Do I need to use Make?**
A: No, it's optional. You can use cargo commands directly. Make just provides convenience.

**Q: What's the difference between `make all` and `make ci`?**
A: `make all` runs fmt, lint, test, build, and doc. `make ci` runs verify (fmt-check, lint, test) plus build and doc.

**Q: Can I run Make targets in parallel?**
A: Yes, use `make -j4` to run with 4 parallel jobs.

**Q: Why use Make instead of just shell scripts?**
A: Make is standardized, widely known, and provides better dependency tracking and parallel execution.

**Q: Does Make work on Windows?**
A: Yes, but you may need to install it separately or use WSL.

## Resources

- [GNU Make Manual](https://www.gnu.org/software/make/manual/)
- [Make Tutorial](https://makefiletutorial.com/)
- [Rust Makefile Best Practices](https://github.com/rust-lang/rust/blob/master/Makefile.in)

---

**Quick Start:** Run `make help` to see all available commands!
