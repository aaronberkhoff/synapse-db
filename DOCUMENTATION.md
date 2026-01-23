# Documentation Overview

This document provides an overview of the Synapse DB documentation system.

## Documentation Structure

Synapse DB has three types of documentation:

### 1. mdBook User Guide (book/)

**Purpose**: Comprehensive user-facing documentation
**Technology**: [mdBook](https://rust-lang.github.io/mdBook/)
**Output**: Static website at `https://aaronberkhoff.github.io/synapse-db`

#### Building

```bash
# Install mdBook
cargo install mdbook

# Build and serve with live reload
mdbook serve

# Build only
mdbook build
```

#### Contents

- **Getting Started**: Installation, quick start, configuration
- **User Guide**: In-depth feature documentation
- **API Reference**: High-level API overview
- **Development**: Contributing, architecture, testing
- **Appendix**: Changelog, FAQ, glossary

See [book/README.md](./book/README.md) for details.

### 2. Rust API Documentation (cargo doc)

**Purpose**: Auto-generated API documentation from doc comments
**Technology**: rustdoc
**Output**: Integrated into mdBook at `/api-docs/`

#### Building

```bash
# Build API docs
cargo doc --no-deps --all-features

# Build and open in browser
cargo doc --no-deps --all-features --open
```

#### Writing Doc Comments

```rust
/// Short description of the function.
///
/// # Arguments
///
/// * `param` - Description of parameter
///
/// # Examples
///
/// ```
/// use synapse_db::Storage;
/// let storage = Storage::new();
/// ```
///
/// # Panics
///
/// When this function panics (if applicable)
///
/// # Errors
///
/// When this function returns an error (if applicable)
pub fn example_function(param: &str) -> Result<(), String> {
    // implementation
}
```

### 3. Inline Code Comments

**Purpose**: Explain complex code logic
**Guidelines**:
- Comment why, not what
- Keep comments updated with code
- Use `//` for line comments, `/* */` for block comments

## Changelog Automation

Changelogs are automatically generated from git commits using [git-cliff](https://git-cliff.org/).

### Configuration

Configured in [cliff.toml](./cliff.toml)

### Generating Changelog

```bash
# Manual generation
./scripts/generate-changelog.sh

# Or directly
cargo install git-cliff
git-cliff --output CHANGELOG.md
```

### Commit Message Format

Use [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

**Types**:
- `feat`: New feature → Appears in "Features" section
- `fix`: Bug fix → Appears in "Bug Fixes" section
- `docs`: Documentation → Appears in "Documentation" section
- `perf`: Performance → Appears in "Performance" section
- `refactor`: Code refactoring → Appears in "Refactor" section
- `style`: Code styling → Appears in "Styling" section
- `test`: Tests → Appears in "Testing" section
- `chore`: Maintenance → Appears in "Miscellaneous" section

**Examples**:
```bash
git commit -m "feat: add query caching"
git commit -m "fix: resolve memory leak in storage"
git commit -m "docs: update API documentation"
git commit -m "perf(query): optimize query execution"
```

### Changelog Workflow

1. Developers commit using conventional commits
2. On release, run `./scripts/generate-changelog.sh`
3. Changelog is updated in:
   - `CHANGELOG.md` (root)
   - `book/changelog.md` (documentation)
4. Commit and tag the release
5. CI automatically publishes updated documentation

## CI/CD Documentation Pipeline

Configured in [.github/workflows/docs.yml](./.github/workflows/docs.yml)

### On Pull Request

1. Build mdBook to verify no errors
2. Build cargo doc to verify doc comments compile
3. Check for broken links (best effort)
4. Upload documentation artifact

### On Push to Main

All the above, plus:
1. Generate changelog from git history
2. Deploy to GitHub Pages

### Manual Trigger

You can manually trigger documentation builds via:
- GitHub Actions UI
- `workflow_dispatch` event

## Documentation Workflow

### Adding New Features

When adding a new feature:

1. **Write code with doc comments**:
   ```rust
   /// Description of new feature
   pub fn new_feature() {}
   ```

2. **Update mdBook user guide**:
   - Add page in `book/` if major feature
   - Update existing page if enhancement
   - Add examples

3. **Update SUMMARY.md** if you added new pages

4. **Update CHANGELOG.md** manually or wait for auto-generation

5. **Test locally**:
   ```bash
   mdbook build
   cargo doc
   ```

6. **Commit with conventional commit message**:
   ```bash
   git commit -m "feat: add awesome feature"
   ```

### Updating Documentation

To update existing documentation:

1. **Edit markdown files** in `book/`

2. **Preview changes**:
   ```bash
   mdbook serve
   # Open http://localhost:3000
   ```

3. **Commit changes**:
   ```bash
   git commit -m "docs: update installation guide"
   ```

4. **CI will auto-deploy** on merge to main

## Documentation Best Practices

### Writing Style

- **Be concise**: Get to the point quickly
- **Use examples**: Show, don't just tell
- **Be consistent**: Use same terminology throughout
- **Keep it current**: Update docs when code changes

### Organization

- **Logical flow**: Guide users from simple to complex
- **Cross-linking**: Link related content
- **Clear hierarchy**: Use headings consistently
- **Searchable**: Use descriptive headings and keywords

### Code Examples

- **Test examples**: Ensure all code examples work
- **Realistic**: Use real-world examples, not toy examples
- **Commented**: Explain non-obvious parts
- **Focused**: One concept per example

### API Documentation

- **Every public item**: Document all public APIs
- **Examples**: Include usage examples
- **Parameters**: Document all parameters
- **Errors**: Document error conditions
- **Panics**: Document panic conditions

## Tools and Dependencies

### Required

- **mdBook**: For building user guide
- **Rust/Cargo**: For API documentation
- **git-cliff**: For changelog generation (optional, can be manual)

### Optional

- **cargo-deadlinks**: Check for broken links in docs
- **mdbook-mermaid**: Add diagram support (not currently used)
- **mdbook-toc**: Table of contents (not currently used)

## File Organization

```
synapse-db/
├── book/                    # mdBook source
│   ├── SUMMARY.md          # Table of contents
│   ├── introduction.md     # Home page
│   ├── getting-started/    # Tutorial content
│   ├── guide/              # Feature guides
│   ├── api/                # API overview
│   ├── development/        # Contributing docs
│   ├── changelog.md        # Copy of root CHANGELOG.md
│   ├── faq.md
│   └── glossary.md
├── book.toml               # mdBook configuration
├── cliff.toml              # git-cliff configuration
├── CHANGELOG.md            # Generated changelog
├── CONTRIBUTING.md         # Contribution guide
├── DOCUMENTATION.md        # This file
├── README.md               # Project overview
├── src/                    # Rust code with doc comments
└── .github/
    └── workflows/
        └── docs.yml        # Documentation CI/CD
```

## Viewing Documentation

### Local Development

```bash
# User guide
mdbook serve
# Open http://localhost:3000

# API docs
cargo doc --open
```

### Online

- **User Guide**: https://aaronberkhoff.github.io/synapse-db
- **API Docs**: https://aaronberkhoff.github.io/synapse-db/api-docs/
- **Source**: https://github.com/aaronberkhoff/synapse-db

## Troubleshooting

### mdBook build fails

```bash
# Ensure mdBook is installed
cargo install mdbook

# Check for syntax errors in SUMMARY.md
mdbook build

# Clean and rebuild
mdbook clean
mdbook build
```

### cargo doc fails

```bash
# Check for doc comment syntax errors
cargo doc

# Fix clippy warnings in doc comments
cargo clippy --all-targets
```

### Changelog generation fails

```bash
# Ensure git-cliff is installed
cargo install git-cliff

# Check cliff.toml for syntax errors
git-cliff --output CHANGELOG.md
```

### GitHub Pages not updating

- Check GitHub Actions workflow logs
- Ensure GitHub Pages is enabled in repository settings
- Verify workflow has `contents: write` permission

## Resources

- [mdBook Guide](https://rust-lang.github.io/mdBook/)
- [rustdoc Guide](https://doc.rust-lang.org/rustdoc/)
- [git-cliff Documentation](https://git-cliff.org/)
- [Conventional Commits](https://www.conventionalcommits.org/)
- [Markdown Guide](https://www.markdownguide.org/)

## Getting Help

- Check this document
- See [book/README.md](./book/README.md) for mdBook specifics
- See [CONTRIBUTING.md](./CONTRIBUTING.md) for contribution guidelines
- Open an issue on [GitHub](https://github.com/aaronberkhoff/synapse-db/issues)

---

**Maintaining Documentation**: Documentation is code. Keep it tested, reviewed, and updated just like your Rust code.
