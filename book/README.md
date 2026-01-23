# Synapse DB Documentation

This directory contains the source for the Synapse DB documentation, built with [mdBook](https://rust-lang.github.io/mdBook/).

## Quick Start

### Building Locally

Install mdBook:

```bash
cargo install mdbook
```

Build the documentation:

```bash
mdbook build
```

Serve locally with live reload:

```bash
mdbook serve
```

Then open http://localhost:3000 in your browser.

## Structure

```
book/
├── SUMMARY.md              # Table of contents (mdBook structure)
├── introduction.md         # Home page
├── getting-started/
│   ├── installation.md
│   ├── quick-start.md
│   └── configuration.md
├── guide/
│   ├── overview.md
│   ├── storage.md
│   ├── query-engine.md
│   ├── examples.md
│   └── best-practices.md
├── api/
│   ├── overview.md
│   ├── storage.md
│   └── query.md
├── development/
│   ├── contributing.md
│   ├── architecture.md
│   ├── testing.md
│   ├── release.md
│   ├── cicd.md
│   └── hooks.md
├── changelog.md            # Generated from ../CHANGELOG.md
├── faq.md
└── glossary.md
```

## Writing Documentation

### Adding a New Page

1. Create the markdown file in the appropriate directory
2. Add it to `SUMMARY.md` in the correct location
3. Write your content
4. Build and preview locally

### Markdown Syntax

mdBook supports standard Markdown plus these features:

#### Code Blocks

```rust
use synapse_db::Storage;

fn main() {
    let storage = Storage::new();
}
```

#### Admonitions (Info/Warning/Note)

Not natively supported, but you can use blockquotes:

> **Note**: This is an important note

> **Warning**: Be careful with this

#### Links

```markdown
[Link to another page](./guide/overview.md)
[External link](https://rust-lang.org)
```

#### Images

```markdown
![Alt text](./images/diagram.png)
```

## Configuration

The documentation is configured in `../book.toml`. Key settings:

- `title`: Book title
- `authors`: Authors list
- `src`: Source directory (this directory)
- `build-dir`: Output directory (`target/book`)
- `output.html.*`: HTML output settings

## Changelog

The changelog is automatically generated from git commits using [git-cliff](https://git-cliff.org/).

To update the changelog:

```bash
./scripts/generate-changelog.sh
```

This generates `../CHANGELOG.md` and copies it to `book/changelog.md`.

## Best Practices

### Writing Guidelines

1. **Be concise**: Get to the point quickly
2. **Use examples**: Show code examples for concepts
3. **Link related content**: Help readers find more information
4. **Keep it current**: Update docs when code changes

### Code Examples

- Test all code examples to ensure they work
- Use realistic examples, not toy examples
- Add comments to explain non-obvious parts
- Keep examples focused on one concept

### Navigation

- Organize content logically in SUMMARY.md
- Use descriptive titles
- Group related pages together
- Keep hierarchy shallow (2-3 levels max)

## mdBook Commands

```bash
# Build the book
mdbook build

# Serve with live reload
mdbook serve

# Serve on custom port
mdbook serve --port 8080

# Open in browser
mdbook serve --open

# Clean build directory
mdbook clean

# Test code examples (requires mdbook-test)
mdbook test
```

## CI/CD

Documentation is automatically built and deployed via GitHub Actions:

- **On PR**: Build to verify no errors
- **On merge to main**: Build and deploy to GitHub Pages

See `.github/workflows/docs.yml` for details.

## Troubleshooting

### Build Errors

```bash
# Ensure mdBook is installed
mdbook --version

# Clean and rebuild
mdbook clean
mdbook build
```

### Missing Pages

Check that the page is listed in `SUMMARY.md`.

### Broken Links

Use mdBook's built-in link checker:

```bash
mdbook build
# Check the build output for warnings
```

## Resources

- [mdBook Documentation](https://rust-lang.github.io/mdBook/)
- [mdBook Guide](https://rust-lang.github.io/mdBook/guide/creating.html)
- [Markdown Guide](https://www.markdownguide.org/)

## Contributing

See [development/contributing.md](./development/contributing.md) for contribution guidelines.
