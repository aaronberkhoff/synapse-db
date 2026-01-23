# Git Hooks

This directory contains Git hooks for the Synapse DB project.

## Installation

Run from the project root:

```bash
./scripts/install-hooks.sh
```

## Available Hooks

- **pre-commit**: Automatically formats and lints code before commit
- **commit-msg**: Validates commit message follows conventional commits format
- **prepare-commit-msg**: Provides helpful commit message template

## Documentation

For detailed documentation, see [../.github/HOOKS.md](../.github/HOOKS.md)

## Bypassing Hooks

Not recommended, but sometimes necessary:

```bash
git commit --no-verify
```

## Testing

To test hooks without committing:

```bash
# Test pre-commit hook
./hooks/pre-commit

# Test commit message validation
echo "invalid message" | ./hooks/commit-msg /dev/stdin
echo "feat: valid message" | ./hooks/commit-msg /dev/stdin
```
