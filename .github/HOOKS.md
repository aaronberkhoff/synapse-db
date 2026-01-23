# Git Hooks Documentation

This project uses Git hooks to maintain code quality and consistency. Hooks run automatically at specific Git events to enforce standards before code is committed.

## 📋 Available Hooks

### 1. Pre-Commit Hook

**Triggers**: Before each commit
**Purpose**: Automatically format and lint code

**What it does**:
1. ✅ Runs `cargo fmt` to format all Rust code
2. ✅ Runs `cargo clippy --fix` to auto-fix linting issues
3. ✅ Automatically stages fixed files
4. ✅ Runs final clippy check to ensure no warnings remain
5. ❌ Blocks commit if unfixable warnings exist

**Example output**:
```
🔍 Running pre-commit checks...

📝 Formatting code with rustfmt...
  ✓ Formatted and staged: src/main.rs
  ✓ Formatted and staged: src/lib.rs

🔧 Running clippy with auto-fix...
  ✓ Fixed and staged: src/query.rs

🔍 Final clippy check...

✅ All pre-commit checks passed!
```

---

### 2. Commit-Msg Hook

**Triggers**: Before commit message is saved
**Purpose**: Enforce conventional commit message format

**Required format**:
```
<type>[optional scope]: <description>

[optional body]

[optional footer]
```

**Valid types**:
- `feat` - New feature
- `fix` - Bug fix
- `docs` - Documentation changes
- `style` - Code style (formatting)
- `refactor` - Code refactoring
- `perf` - Performance improvement
- `test` - Tests
- `build` - Build system
- `ci` - CI/CD changes
- `chore` - Maintenance

**Valid examples**:
```
feat: add query caching
fix: resolve memory leak in storage
docs(readme): update installation instructions
perf(storage): optimize disk I/O operations
```

**Invalid examples**:
```
Added new feature          ❌ No type prefix
feat add caching          ❌ Missing colon
fix: bug                  ❌ Description too short
update: docs              ❌ Invalid type
```

---

### 3. Prepare-Commit-Msg Hook

**Triggers**: When opening commit message editor
**Purpose**: Provide helpful template

**What it does**:
- Provides a commit message template with examples
- Reminds you of conventional commit format
- Shows available types and their meanings

---

## 🚀 Installation

### First Time Setup

Install hooks when you first clone the repository:

```bash
./scripts/install-hooks.sh
```

### Updating Hooks

If hooks are updated in the repository, reinstall them:

```bash
./scripts/install-hooks.sh
```

### Automatic Installation

Add this to your `~/.gitconfig` to auto-install hooks:

```ini
[init]
    templateDir = ~/.git-templates
```

Then create the template:
```bash
mkdir -p ~/.git-templates/hooks
# Copy hooks to template directory
```

---

## 🔧 Usage

### Normal Commits

Just commit as usual - hooks run automatically:

```bash
git add .
git commit -m "feat: add new feature"
```

The pre-commit hook will:
1. Auto-format your code
2. Auto-fix linting issues
3. Stage the fixes
4. Verify everything passes

### Bypassing Hooks

**⚠️ Not recommended**, but sometimes necessary:

```bash
# Skip all hooks
git commit --no-verify -m "emergency fix"

# Or use the shorthand
git commit -n -m "emergency fix"
```

**When to bypass**:
- Emergency hotfixes
- WIP commits on feature branches
- Fixing hook issues themselves

**Never bypass when**:
- Merging to main
- Creating pull requests
- Making releases

---

## 🛠️ Troubleshooting

### Hook Not Running

**Check if installed**:
```bash
ls -la .git/hooks/pre-commit
```

**Reinstall**:
```bash
./scripts/install-hooks.sh
```

**Check permissions**:
```bash
chmod +x .git/hooks/pre-commit
```

### Pre-Commit Fails

**Error: "Clippy found warnings that couldn't be auto-fixed"**

Solution:
```bash
# See what warnings exist
cargo clippy --all-targets --all-features

# Fix them manually, then commit again
```

**Error: "cargo: command not found"**

Solution:
```bash
# Load Rust environment
source "$HOME/.cargo/env"

# Or add to your shell profile
echo 'source "$HOME/.cargo/env"' >> ~/.bashrc
```

### Commit Message Rejected

**Error: "Invalid commit message format"**

The hook shows examples of valid formats. Follow the conventional commits pattern:

```bash
# Bad
git commit -m "fixed bug"

# Good
git commit -m "fix: resolve null pointer in query parser"
```

### Hook Takes Too Long

If the pre-commit hook is slow:

1. **Only commit changed files** (hook only checks staged files)
2. **Use incremental builds** (Cargo caches build artifacts)
3. **Consider faster checks**:
   ```bash
   # Use cargo check instead of clippy for WIP commits
   git commit --no-verify -m "wip: work in progress"
   # Then run full checks before PR
   ./scripts/pre-push-check.sh
   ```

---

## 🎨 Customization

### Disable Specific Checks

Edit `hooks/pre-commit` to comment out checks:

```bash
# Disable clippy auto-fix
# cargo clippy --fix --allow-staged --allow-dirty

# Disable formatting
# cargo fmt --all
```

### Add Custom Checks

Add your own checks to `hooks/pre-commit`:

```bash
# Example: Check for debug statements
if git diff --cached | grep -E "println!|dbg!"; then
    echo "Warning: Debug statements found"
fi
```

### Adjust Clippy Strictness

Edit the clippy command in `hooks/pre-commit`:

```bash
# Allow warnings (not recommended)
cargo clippy --fix --allow-staged --allow-dirty

# Deny specific lints
cargo clippy -- -D clippy::unwrap_used

# Allow specific lints
cargo clippy -- -A clippy::too_many_arguments
```

---

## 📊 Hook Performance

Typical execution times on average hardware:

| Hook | Duration | Can Skip |
|------|----------|----------|
| prepare-commit-msg | <0.1s | No (template only) |
| pre-commit | 2-10s | Yes (--no-verify) |
| commit-msg | <0.1s | Yes (--no-verify) |

**Performance tips**:
- Hooks use cargo caching (first run slower)
- Only checks staged files
- Clippy auto-fix is incremental
- Consider `cargo check` instead of `clippy` for large projects

---

## 🔍 Testing Hooks

### Test Pre-Commit Hook

Create a poorly formatted file:

```bash
# Create a test file with bad formatting
cat > src/test.rs << 'EOF'
pub fn    bad_formatting(  )   {
    let x=1+2;
        println!("test");
}
EOF

# Try to commit it
git add src/test.rs
git commit -m "test: formatting"

# Hook should auto-format and fix it
```

### Test Commit-Msg Hook

Try invalid commit messages:

```bash
# Should fail
git commit --allow-empty -m "bad message"

# Should succeed
git commit --allow-empty -m "test: valid message"
```

---

## 📝 Best Practices

1. **Install hooks immediately** after cloning
2. **Don't bypass hooks** unless absolutely necessary
3. **Keep hooks updated** by running install script after pulling changes
4. **Run full checks** before pushing: `./scripts/pre-push-check.sh`
5. **Use descriptive commit messages** following conventional commits
6. **Commit frequently** - hooks are fast with incremental changes

---

## 🔗 Related Files

- [hooks/pre-commit](../hooks/pre-commit) - Pre-commit hook script
- [hooks/commit-msg](../hooks/commit-msg) - Commit message validator
- [hooks/prepare-commit-msg](../hooks/prepare-commit-msg) - Message template
- [scripts/install-hooks.sh](../scripts/install-hooks.sh) - Installation script
- [scripts/pre-push-check.sh](../scripts/pre-push-check.sh) - Pre-push validation

---

## 📚 Resources

- [Conventional Commits](https://www.conventionalcommits.org/)
- [Git Hooks Documentation](https://git-scm.com/docs/githooks)
- [Cargo Clippy](https://github.com/rust-lang/rust-clippy)
- [Rustfmt](https://github.com/rust-lang/rustfmt)

---

**Last Updated**: 2026-01-22
