# Contributing to Synapse DB

Thank you for your interest in contributing to Synapse DB! This document provides guidelines and information about our development process.

## Development Setup

### Quick Setup (Recommended)

Run the automated setup script:

```bash
git clone https://github.com/ajber/synapse-db.git
cd synapse-db
./scripts/setup.sh
```

This script automatically:
- ✅ Installs/updates Rust and Cargo
- ✅ Installs development tools (mdBook, git-cliff, cargo-audit, cargo-watch)
- ✅ Sets up Git hooks
- ✅ Builds the project (debug and release)
- ✅ Runs tests
- ✅ Builds documentation

### Manual Setup

If the automated script doesn't work or you prefer manual setup:

1. **Install Rust**: Get the latest stable Rust toolchain from [rustup.rs](https://rustup.rs/)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Clone the repository**:
   ```bash
   git clone https://github.com/ajber/synapse-db.git
   cd synapse-db
   ```

3. **Install development tools**:
   ```bash
   cargo install mdbook git-cliff cargo-audit cargo-watch
   ```

4. **Install Git hooks**:
   ```bash
   ./scripts/install-hooks.sh
   ```

   This installs hooks that automatically:
   - Format code with `rustfmt` before commits
   - Fix linting issues with `clippy --fix` before commits
   - Validate commit message format

   See [.github/HOOKS.md](.github/HOOKS.md) for more details.

5. **Build and test**:
   ```bash
   cargo build
   cargo test
   ```

## Development Workflow

### Before Committing

If you installed Git hooks, formatting and linting happen automatically on commit. Otherwise, run these checks manually:

```bash
# Format your code
cargo fmt

# Run linter
cargo clippy --all-targets --all-features -- -D warnings

# Run all tests
cargo test --all-features

# Check for security vulnerabilities
cargo install cargo-audit
cargo audit
```

**Before pushing**, run the comprehensive check script:

```bash
./scripts/pre-push-check.sh
```

This runs all CI checks locally to ensure your push will pass in CI.

### Making Changes

1. **Create a branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes** and commit with clear messages following [Conventional Commits](https://www.conventionalcommits.org/):
   ```bash
   git commit -m "feat: add user authentication"
   git commit -m "fix: resolve memory leak in storage"
   git commit -m "docs: update API documentation"
   ```

   The commit-msg hook will validate your message format. Valid types: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `build`, `ci`, `chore`.

3. **Push and create a Pull Request**:
   ```bash
   git push origin feature/your-feature-name
   ```

### Pull Request Guidelines

- Write clear, descriptive PR titles and descriptions
- Reference any related issues
- Ensure all CI checks pass
- Update documentation if needed
- Add tests for new functionality

## CI/CD Process

### Continuous Integration

When you open a PR, the following checks run automatically:

1. **Test Suite** (`test` job)
   - Runs on Linux, macOS, Windows
   - Tests with stable and beta Rust
   - Includes unit tests, integration tests, and doc tests

2. **Code Quality** (`fmt`, `clippy` jobs)
   - Formatting must match `rustfmt` standards
   - No clippy warnings allowed

3. **Code Coverage** (`coverage` job)
   - Generates coverage report
   - Uploads to Codecov
   - Helps identify untested code

4. **Security** (`security-audit` job)
   - Checks dependencies for known vulnerabilities
   - Fails if critical vulnerabilities found

5. **Build** (`build` job)
   - Ensures release builds succeed on all platforms

### All Checks Must Pass

Pull requests cannot be merged until:
- ✅ All CI checks pass
- ✅ Code review is approved
- ✅ Branch is up to date with main

## Coding Standards

### Code Style

- Use `rustfmt` defaults (see [rustfmt.toml](rustfmt.toml))
- Follow Rust naming conventions
- Write clear, self-documenting code
- Add comments for complex logic

### Testing

- Write unit tests for all public functions
- Add integration tests for end-to-end scenarios
- Aim for high code coverage (target: >80%)
- Include edge cases and error conditions

Example:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_works() {
        // Arrange
        let input = setup_test_data();

        // Act
        let result = your_function(input);

        // Assert
        assert_eq!(result, expected_output);
    }
}
```

### Documentation

- Add doc comments to all public items
- Include examples in doc comments
- Keep README.md up to date
- Document breaking changes

Example:
```rust
/// Executes a query against the database.
///
/// # Arguments
///
/// * `query` - The SQL query string to execute
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error message on failure.
///
/// # Examples
///
/// ```
/// use synapse_db::QueryEngine;
///
/// let engine = QueryEngine::new();
/// engine.execute("SELECT 1").unwrap();
/// ```
pub fn execute(&self, query: &str) -> Result<(), String> {
    // implementation
}
```

## Release Process

Releases are automated via GitHub Actions:

1. **Update version** in `Cargo.toml`:
   ```toml
   [package]
   version = "0.2.0"
   ```

2. **Update CHANGELOG** (if you have one) with release notes

3. **Create and push a tag**:
   ```bash
   git tag v0.2.0
   git push origin v0.2.0
   ```

4. **GitHub Actions will**:
   - Build binaries for all platforms
   - Create a GitHub release
   - Publish to crates.io (if credentials are configured)

## Getting Help

- Open an issue for bugs or feature requests
- Check existing issues and PRs first
- Join discussions in GitHub Discussions (if enabled)

## Code of Conduct

- Be respectful and constructive
- Welcome newcomers
- Focus on the code, not the person
- Follow the Rust Code of Conduct

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
