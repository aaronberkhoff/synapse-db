# Development Environment Setup

Quick reference for setting up your Synapse DB development environment.

## One-Line Setup

```bash
./scripts/setup.sh
```

This automated script handles everything you need to start developing.

## What Gets Installed

### 1. Rust Toolchain
- Latest stable Rust
- Cargo package manager
- Rustfmt (code formatter)
- Clippy (linter)

### 2. Development Tools
- **mdBook** - Documentation builder
- **git-cliff** - Automatic changelog generation
- **cargo-audit** - Security vulnerability scanner
- **cargo-watch** - Auto-rebuild on file changes (optional)

### 3. Git Hooks
- **pre-commit** - Auto-format and lint code
- **commit-msg** - Validate conventional commit messages
- **prepare-commit-msg** - Helpful commit message template

### 4. Project Build
- Debug build (`target/debug/`)
- Release build (`target/release/`)
- Test suite execution
- Documentation generation

## Manual Installation

If the automated script fails or you prefer manual setup:

### Step 1: Install Rust

**Linux/macOS:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**Windows:**
Download from [rustup.rs](https://rustup.rs/)

### Step 2: Install Tools

```bash
# Essential tools
cargo install mdbook git-cliff cargo-audit

# Optional but recommended
cargo install cargo-watch
```

### Step 3: Setup Git Hooks

```bash
./scripts/install-hooks.sh
```

### Step 4: Build Project

```bash
# Build and test
cargo build
cargo test

# Build documentation
mdbook build
cargo doc --no-deps --all-features
```

## Verifying Installation

Check that everything is installed correctly:

```bash
# Rust toolchain
rustc --version
cargo --version

# Development tools
mdbook --version
git-cliff --version
cargo audit --version
cargo watch --version

# Build status
cargo build
cargo test
```

## System Requirements

### Minimum Requirements
- **OS**: Linux, macOS, Windows (WSL recommended)
- **RAM**: 4GB (8GB recommended)
- **Disk**: 2GB free space
- **Internet**: Required for initial setup

### Rust Version
- **Minimum**: Rust 1.70.0
- **Recommended**: Latest stable

## Troubleshooting

### Rust not found after installation

Add Cargo to your PATH:

```bash
# Bash/Zsh
echo 'source $HOME/.cargo/env' >> ~/.bashrc
source ~/.bashrc

# Fish
echo 'source $HOME/.cargo/env.fish' >> ~/.config/fish/config.fish
```

### Build errors

Update Rust and clean build:

```bash
rustup update stable
cargo clean
cargo build
```

### Hook installation fails

Manually copy hooks:

```bash
cp hooks/pre-commit .git/hooks/
cp hooks/commit-msg .git/hooks/
cp hooks/prepare-commit-msg .git/hooks/
chmod +x .git/hooks/*
```

### mdBook not found

Ensure `~/.cargo/bin` is in your PATH:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

### Permission denied on Linux/macOS

Make scripts executable:

```bash
chmod +x scripts/*.sh
chmod +x hooks/*
```

## Post-Setup

After setup completes, you can:

### Start Development
```bash
# Run the application
cargo run

# Auto-rebuild on changes
cargo watch -x run

# Run tests continuously
cargo watch -x test
```

### View Documentation
```bash
# Serve with live reload
mdbook serve
# Open http://localhost:3000

# Open API docs
cargo doc --open
```

### Before Committing
```bash
# Run all checks
./scripts/pre-push-check.sh

# Or manually
cargo fmt
cargo clippy
cargo test
```

## Development Workflow

1. **Create a branch**
   ```bash
   git checkout -b feature/my-feature
   ```

2. **Make changes**
   - Code auto-formats on commit
   - Commit messages validated automatically

3. **Run checks**
   ```bash
   ./scripts/pre-push-check.sh
   ```

4. **Push changes**
   ```bash
   git push origin feature/my-feature
   ```

5. **Create Pull Request**
   - CI runs automatically
   - All checks must pass

## Development Tools Usage

### Code Quality
```bash
cargo fmt              # Format code
cargo clippy           # Lint code
cargo test             # Run tests
cargo check            # Quick compile check
```

### Documentation
```bash
mdbook serve           # User guide (live reload)
mdbook build           # Build user guide
cargo doc --open       # API documentation
./scripts/generate-changelog.sh  # Update changelog
```

### Performance
```bash
cargo build --release  # Optimized build
cargo bench            # Run benchmarks (if configured)
```

### Security
```bash
cargo audit            # Security vulnerabilities
cargo outdated         # Outdated dependencies
```

## Environment Variables

The setup script respects these environment variables:

- `RUSTUP_HOME` - Rustup installation directory
- `CARGO_HOME` - Cargo installation directory (default: `~/.cargo`)

## Uninstalling

To remove Synapse DB development environment:

```bash
# Remove Rust (if desired)
rustup self uninstall

# Remove development tools
cargo uninstall mdbook git-cliff cargo-audit cargo-watch

# Remove repository
cd ..
rm -rf synapse-db
```

## Getting Help

- **Setup Issues**: Check [Troubleshooting](#troubleshooting) section
- **Development Questions**: See [CONTRIBUTING.md](CONTRIBUTING.md)
- **CI/CD**: See [.github/WORKFLOWS.md](.github/WORKFLOWS.md)
- **Git Hooks**: See [.github/HOOKS.md](.github/HOOKS.md)
- **General**: Open an [issue](https://github.com/ajber/synapse-db/issues)

## Next Steps

After setup:

1. Read [CONTRIBUTING.md](CONTRIBUTING.md) - Contribution guidelines
2. Check [book/README.md](book/README.md) - Documentation guide
3. Review [DOCUMENTATION.md](DOCUMENTATION.md) - Documentation system
4. Explore [examples/](examples/) - Example code
5. Join discussions on [GitHub](https://github.com/ajber/synapse-db/discussions)

---

**Setup Time**: ~10-15 minutes depending on internet speed

**Questions?** Open an issue or discussion on GitHub!
