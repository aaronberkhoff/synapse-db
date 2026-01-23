# CI/CD Quick Reference Card

## 🚀 Quick Commands

### Before You Push
```bash
# Run all checks locally
./scripts/pre-push-check.sh

# Or run individually:
cargo fmt           # Format code
cargo clippy        # Lint code
cargo test          # Run tests
cargo build --release
```

### Creating a Release
```bash
# 1. Update version in Cargo.toml
# 2. Commit the version change
git add Cargo.toml
git commit -m "chore: bump version to 0.2.0"

# 3. Create and push tag
git tag v0.2.0
git push origin v0.2.0

# 4. GitHub Actions will automatically:
#    - Build binaries (Linux, macOS, Windows)
#    - Create GitHub release
#    - Publish to crates.io
```

### Manual Workflow Triggers
```bash
# Go to: Actions → Select workflow → Run workflow
# Or use GitHub CLI:
gh workflow run scheduled.yml
```

---

## 📋 Workflow Triggers

| Workflow | Trigger | What It Does |
|----------|---------|--------------|
| **CI** | Push/PR | Tests, lint, coverage, build |
| **PR Checks** | Pull requests | Title validation, dependency review |
| **Release** | Tag `v*.*.*` | Build & publish release |
| **Benchmark** | Push to main/PR | Performance tests |
| **Docs** | Push to main/PR | Build documentation |
| **Scheduled** | Daily 2 AM UTC | Dependency checks, nightly tests |
| **Dependabot** | Weekly (Mon) | Auto dependency PRs |

---

## ✅ CI Checks Checklist

Your PR must pass:
- [ ] Tests (Linux, macOS, Windows)
- [ ] Code formatting (`cargo fmt`)
- [ ] Clippy lints (no warnings)
- [ ] Security audit (no vulnerabilities)
- [ ] Release build succeeds
- [ ] Code review approved

---

## 🏷️ Commit/PR Title Format

Use conventional commits:
- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation
- `style:` - Code style (formatting)
- `refactor:` - Code refactoring
- `perf:` - Performance improvement
- `test:` - Tests
- `build:` - Build system
- `ci:` - CI/CD changes
- `chore:` - Maintenance

**Examples**:
```
feat: add query caching
fix: resolve memory leak in storage
docs: update README with examples
```

---

## 🔧 Common Issues & Fixes

### ❌ Format check failed
```bash
cargo fmt
git add .
git commit --amend --no-edit
git push --force-with-lease
```

### ❌ Clippy warnings
```bash
cargo clippy --fix --allow-dirty
# Review changes, then commit
```

### ❌ Tests failing
```bash
cargo test --verbose
# Fix the tests, then push
```

### ❌ Security vulnerabilities
```bash
cargo audit
cargo update
# Or update specific package:
cargo update -p package-name
```

### ❌ Release build failed
```bash
cargo build --release
# Fix errors, then push
```

---

## 📊 Monitoring

### View Workflow Status
- **Dashboard**: https://github.com/aaronberkhoff/synapse-db/actions
- **Badges**: Check README badges
- **Notifications**: Watch repo for alerts

### Code Coverage
- **Codecov**: https://codecov.io/gh/aaronberkhoff/synapse-db
- **Target**: >80% coverage

### Dependencies
- **Security**: Check GitHub Security tab
- **Updates**: Review Dependabot PRs weekly

---

## 🔐 Required Secrets

Set in: `Settings → Secrets and variables → Actions`

- `CARGO_REGISTRY_TOKEN` - For crates.io publishing
  - Get from: https://crates.io/settings/tokens
  - Scope: Publish new versions

---

## 📖 Documentation

- **Full Workflows Guide**: [.github/WORKFLOWS.md](.github/WORKFLOWS.md)
- **Contributing**: [CONTRIBUTING.md](../CONTRIBUTING.md)
- **README**: [README.md](../README.md)

---

## 🆘 Get Help

1. Check workflow logs in Actions tab
2. Review [WORKFLOWS.md](.github/WORKFLOWS.md)
3. Open an issue with `ci` label
4. Ask in discussions

---

## 💡 Pro Tips

1. **Use cargo-watch** for auto-reload during development:
   ```bash
   cargo install cargo-watch
   cargo watch -x test
   ```

2. **Run specific test**:
   ```bash
   cargo test test_name
   ```

3. **Skip CI for docs-only changes**:
   ```bash
   git commit -m "docs: update README [skip ci]"
   ```

4. **Check what will be included in crate**:
   ```bash
   cargo package --list
   ```

5. **Dry run release**:
   ```bash
   cargo publish --dry-run
   ```

---

**Last Updated**: 2026-01-22
**CI/CD Stack**: GitHub Actions, Rust Stable 1.92.0
