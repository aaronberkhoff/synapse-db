# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial project setup
- Storage layer implementation (placeholder)
- Query engine implementation (placeholder)
- Comprehensive CI/CD pipeline with GitHub Actions
- Pre-commit hooks for code formatting and linting
- MkDocs documentation with Material theme
- Automated changelog generation

### Changed
- N/A

### Deprecated
- N/A

### Removed
- N/A

### Fixed
- N/A

### Security
- N/A

## [0.1.0] - 2026-01-22

### Added
- Initial release
- Basic project structure
- Storage module skeleton
- Query engine module skeleton
- Example code
- Integration tests
- Benchmark framework
- GitHub Actions workflows:
  - CI (test, lint, format, security audit)
  - Release automation
  - Documentation build
  - Scheduled checks
  - Pull request validation
- Git hooks:
  - pre-commit (auto-format and lint)
  - commit-msg (validate conventional commits)
  - prepare-commit-msg (template)
- Documentation:
  - README with badges
  - CONTRIBUTING guide
  - License (MIT)
  - CI/CD documentation
  - Hooks documentation

### Changed
- N/A

### Fixed
- N/A

---

## How to Update This Changelog

### For Contributors

When making changes, add entries under the `[Unreleased]` section following these guidelines:

1. **Choose the right category**:
   - `Added` for new features
   - `Changed` for changes in existing functionality
   - `Deprecated` for soon-to-be removed features
   - `Removed` for now removed features
   - `Fixed` for any bug fixes
   - `Security` for vulnerability fixes

2. **Write clear, descriptive entries**:
   ```markdown
   - Added query caching for improved performance (#123)
   - Fixed memory leak in storage layer (#124)
   - Changed default buffer size from 4KB to 8KB
   ```

3. **Reference issues/PRs** when applicable using `(#issue-number)`

4. **Keep entries user-focused** - describe what changed from a user's perspective

### For Maintainers

When creating a release:

1. **Move unreleased changes** to a new version section:
   ```markdown
   ## [0.2.0] - 2026-02-15
   ```

2. **Update the links** at the bottom of the file

3. **Commit the changelog** with the release

### Automation

This changelog can be automatically generated from conventional commits using:

```bash
# Generate changelog from commits
./scripts/generate-changelog.sh

# Or use git-cliff (if installed)
git cliff --output CHANGELOG.md
```

---

## Version Links

[Unreleased]: https://github.com/aaronberkhoff/synapse-db/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/aaronberkhoff/synapse-db/releases/tag/v0.1.0
