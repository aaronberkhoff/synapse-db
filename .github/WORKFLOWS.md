# GitHub Actions Workflows

This document describes all CI/CD workflows configured for Synapse DB.

## Workflow Overview

```
┌─────────────────────────────────────────────────────────────┐
│                     Synapse DB CI/CD                        │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  On Push/PR:                                               │
│  ├─ ci.yml           (Main CI pipeline)                    │
│  ├─ pr.yml           (PR-specific checks)                  │
│  ├─ docs.yml         (Documentation build)                 │
│  └─ benchmark.yml    (Performance tests)                   │
│                                                             │
│  On Tag (v*.*.*):                                          │
│  └─ release.yml      (Build & publish releases)            │
│                                                             │
│  Scheduled (Daily):                                        │
│  └─ scheduled.yml    (Dependency & nightly checks)         │
│                                                             │
│  Automated:                                                │
│  └─ dependabot.yml   (Dependency updates)                  │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## Detailed Workflow Descriptions

### 1. CI Workflow (`ci.yml`)

**Triggers**: Push to main/develop, Pull requests
**Purpose**: Main continuous integration pipeline

**Jobs**:
- **test** - Run test suite
  - Matrix: Ubuntu, Windows, macOS × Stable, Beta Rust
  - Runs all unit, integration, and doc tests
  - Uses cargo caching for faster builds

- **fmt** - Code formatting check
  - Ensures code follows `rustfmt` standards
  - Fails if code is not formatted

- **clippy** - Linting
  - Runs with `-D warnings` (treat warnings as errors)
  - Checks all targets and features

- **coverage** - Code coverage
  - Uses `cargo-tarpaulin` to generate coverage
  - Uploads to Codecov
  - Helps identify untested code

- **security-audit** - Security scanning
  - Uses RustSec advisory database
  - Checks dependencies for vulnerabilities

- **build** - Build artifacts
  - Creates release builds for all platforms
  - Uploads build artifacts

**Status**: Must pass for PR merge

---

### 2. Release Workflow (`release.yml`)

**Triggers**: Git tags matching `v*.*.*` (e.g., `v1.2.3`)
**Purpose**: Automated release process

**Jobs**:
- **create-release** - Create GitHub release
  - Extracts version from tag
  - Creates release draft

- **build-release** - Build platform binaries
  - Matrix builds for:
    - Linux (GNU and musl)
    - macOS (Intel and ARM)
    - Windows
  - Strips binaries for smaller size
  - Creates `.tar.gz` (Unix) or `.zip` (Windows) archives
  - Uploads as release assets

- **publish-crate** - Publish to crates.io
  - Requires `CARGO_REGISTRY_TOKEN` secret
  - Makes crate available on crates.io

**How to trigger**:
```bash
git tag v0.1.0
git push origin v0.1.0
```

---

### 3. Pull Request Workflow (`pr.yml`)

**Triggers**: Pull request events
**Purpose**: Additional PR-specific validation

**Jobs**:
- **pr-checks** - Validation checks
  - Enforces conventional commit PR titles
  - Detects new TODO/FIXME comments
  - Checks for large files (>1MB)
  - Detects potential breaking changes

- **dependency-review** - Dependency analysis
  - Reviews new dependencies
  - Checks for security issues
  - Fails on moderate+ severity vulnerabilities

---

### 4. Benchmark Workflow (`benchmark.yml`)

**Triggers**: Push to main, Pull requests
**Purpose**: Performance regression detection

**Jobs**:
- **benchmark** - Run benchmarks
  - Executes `cargo bench`
  - Stores results for comparison
  - Alerts on >200% performance degradation
  - Comments on PR if performance degrades

**Note**: Requires criterion setup in Cargo.toml

---

### 5. Documentation Workflow (`docs.yml`)

**Triggers**: Push to main, Pull requests
**Purpose**: Build and publish documentation

**Jobs**:
- **docs** - Build rustdoc
  - Generates API documentation
  - Checks for broken links
  - Deploys to GitHub Pages (main branch only)

**Access docs at**: `https://<username>.github.io/<repo>/`

---

### 6. Scheduled Workflow (`scheduled.yml`)

**Triggers**: Daily at 2 AM UTC, Manual dispatch
**Purpose**: Routine maintenance checks

**Jobs**:
- **outdated-dependencies** - Check for updates
  - Uses `cargo-outdated`
  - Identifies newer package versions

- **unused-dependencies** - Detect unused deps
  - Uses `cargo-udeps` on nightly
  - Finds dependencies that can be removed

- **nightly-test** - Test with Rust nightly
  - Ensures compatibility with latest Rust
  - Catches potential future issues early

- **msrv-check** - Minimum Rust version
  - Tests with Rust 1.70.0
  - Validates MSRV claims

---

### 7. Dependabot (`dependabot.yml`)

**Triggers**: Weekly (Mondays)
**Purpose**: Automated dependency updates

**Configuration**:
- **Cargo dependencies**: Weekly checks
  - Opens PRs for outdated crates
  - Max 10 open PRs
  - Labels: `dependencies`, `rust`

- **GitHub Actions**: Weekly checks
  - Updates action versions
  - Max 5 open PRs
  - Labels: `dependencies`, `github-actions`

---

## Required Secrets

Configure these in GitHub Settings → Secrets:

| Secret | Purpose | Required |
|--------|---------|----------|
| `GITHUB_TOKEN` | Automatic, provided by GitHub | Auto |
| `CARGO_REGISTRY_TOKEN` | Publishing to crates.io | Optional |
| `CODECOV_TOKEN` | Code coverage uploads | Optional |

### Getting `CARGO_REGISTRY_TOKEN`:
1. Go to https://crates.io/settings/tokens
2. Create new token
3. Add to GitHub repo secrets

---

## Workflow Status Badges

Add these to your README:

```markdown
[![CI](https://github.com/ajber/synapse-db/workflows/CI/badge.svg)](https://github.com/ajber/synapse-db/actions/workflows/ci.yml)
[![Release](https://github.com/ajber/synapse-db/workflows/Release/badge.svg)](https://github.com/ajber/synapse-db/actions/workflows/release.yml)
[![codecov](https://codecov.io/gh/ajber/synapse-db/branch/main/graph/badge.svg)](https://codecov.io/gh/ajber/synapse-db)
```

---

## Local Development

Before pushing, run all checks locally:

```bash
./scripts/pre-push-check.sh
```

This script runs:
- Code formatting check
- Clippy lints
- All tests
- Release build
- Security audit (if installed)

---

## Troubleshooting

### CI Failing?

1. **Format issues**: Run `cargo fmt`
2. **Clippy warnings**: Run `cargo clippy --fix`
3. **Test failures**: Run `cargo test` locally
4. **Build errors**: Ensure `cargo build --release` works locally

### Release Not Publishing?

1. Check `CARGO_REGISTRY_TOKEN` secret is set
2. Ensure version in Cargo.toml matches tag
3. Verify crate name isn't already taken on crates.io

### Dependabot PRs Failing?

1. Update local dependencies: `cargo update`
2. Fix any breaking changes
3. Update code if API changed

---

## Customization

### Change Test Matrix

Edit `.github/workflows/ci.yml`:
```yaml
matrix:
  os: [ubuntu-latest]  # Remove platforms
  rust: [stable]       # Remove beta testing
```

### Disable Workflows

Rename or delete workflow files you don't need, or add:
```yaml
on:
  workflow_dispatch:  # Manual trigger only
```

### Add More Platforms

In `release.yml`, add to matrix:
```yaml
- os: ubuntu-latest
  target: aarch64-unknown-linux-gnu
  artifact_name: synapse-db
  asset_name: synapse-db-linux-arm64
```

---

## Performance Considerations

- Caching reduces build times by ~80%
- Parallel jobs run simultaneously
- Matrix builds use GitHub's free minutes
- Consider disabling Windows/macOS for forks

---

## Best Practices

1. **Always run local checks** before pushing
2. **Keep dependencies updated** (review Dependabot PRs)
3. **Monitor security audits** (fix vulnerabilities promptly)
4. **Tag releases** properly (`v` prefix + semver)
5. **Write tests** (maintain >80% coverage)
6. **Document changes** (update README/docs)

---

## Resources

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Rust CI Guide](https://github.com/actions-rs)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [Semantic Versioning](https://semver.org/)
