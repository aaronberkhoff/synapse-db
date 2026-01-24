# Synapse DB Makefile
# Provides convenient commands for development tasks

.PHONY: help setup build test clean fmt lint doc check run release install-hooks pre-push changelog serve watch audit outdated all \
        python-build python-dev python-test python-stubs python-typecheck python-wheel python-clean python-all

# Default target - show help
.DEFAULT_GOAL := help

# Colors for output
BLUE := \033[0;34m
GREEN := \033[0;32m
YELLOW := \033[1;33m
NC := \033[0m # No Color

##@ Help

help: ## Display this help message
	@echo "$(BLUE)Synapse DB - Development Commands$(NC)"
	@echo ""
	@awk 'BEGIN {FS = ":.*##"; printf "Usage:\n  make $(GREEN)<target>$(NC)\n"} /^[a-zA-Z_0-9-]+:.*?##/ { printf "  $(GREEN)%-15s$(NC) %s\n", $$1, $$2 } /^##@/ { printf "\n$(BLUE)%s$(NC)\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

##@ Setup

setup: ## Run complete development environment setup
	@echo "$(BLUE)Running setup script...$(NC)"
	@./scripts/setup.sh

install-hooks: ## Install Git hooks
	@echo "$(BLUE)Installing Git hooks...$(NC)"
	@./scripts/install-hooks.sh
	@echo "$(GREEN)✓ Git hooks installed$(NC)"

##@ Building

build: ## Build the project (debug mode)
	@echo "$(BLUE)Building debug version...$(NC)"
	@cargo build
	@echo "$(GREEN)✓ Build complete$(NC)"

release: ## Build optimized release version
	@echo "$(BLUE)Building release version...$(NC)"
	@cargo build --release
	@echo "$(GREEN)✓ Release build complete$(NC)"

clean: ## Remove build artifacts
	@echo "$(BLUE)Cleaning build artifacts...$(NC)"
	@cargo clean
	@rm -rf target/book
	@echo "$(GREEN)✓ Clean complete$(NC)"

all: fmt lint test build doc ## Run all Rust checks and build

all-with-python: all python-all ## Run all Rust and Python checks

##@ Testing

test: ## Run all tests
	@echo "$(BLUE)Running tests...$(NC)"
	@cargo test
	@echo "$(GREEN)✓ All tests passed$(NC)"

test-verbose: ## Run tests with verbose output
	@echo "$(BLUE)Running tests (verbose)...$(NC)"
	@cargo test -- --nocapture

bench: ## Run benchmarks
	@echo "$(BLUE)Running benchmarks...$(NC)"
	@cargo bench

coverage: ## Generate test coverage report
	@echo "$(BLUE)Generating coverage...$(NC)"
	@cargo install cargo-tarpaulin || true
	@cargo tarpaulin --verbose --all-features --workspace --timeout 120 --out Html
	@echo "$(GREEN)✓ Coverage report generated$(NC)"

##@ Code Quality

fmt: ## Format code with rustfmt
	@echo "$(BLUE)Formatting code...$(NC)"
	@cargo fmt
	@echo "$(GREEN)✓ Code formatted$(NC)"

fmt-check: ## Check code formatting
	@echo "$(BLUE)Checking code formatting...$(NC)"
	@cargo fmt --all -- --check

lint: ## Run clippy linter
	@echo "$(BLUE)Running clippy...$(NC)"
	@cargo clippy --all-targets --all-features -- -D warnings
	@echo "$(GREEN)✓ No linting issues$(NC)"

lint-fix: ## Auto-fix clippy warnings
	@echo "$(BLUE)Auto-fixing clippy warnings...$(NC)"
	@cargo clippy --fix --allow-dirty --allow-staged --all-targets --all-features
	@echo "$(GREEN)✓ Clippy fixes applied$(NC)"

check: ## Quick compile check
	@echo "$(BLUE)Running cargo check...$(NC)"
	@cargo check --all-targets --all-features
	@echo "$(GREEN)✓ Check complete$(NC)"

pre-push: ## Run all pre-push checks
	@echo "$(BLUE)Running pre-push checks...$(NC)"
	@./scripts/pre-push-check.sh

##@ Documentation

doc: ## Build all documentation
	@echo "$(BLUE)Building documentation...$(NC)"
	@mdbook build
	@cargo doc --no-deps --all-features
	@echo "$(GREEN)✓ Documentation built$(NC)"

doc-open: ## Build and open API documentation
	@echo "$(BLUE)Building and opening API docs...$(NC)"
	@cargo doc --no-deps --all-features --open

serve: ## Serve documentation with live reload
	@echo "$(BLUE)Serving documentation at http://localhost:3000$(NC)"
	@mdbook serve

changelog: ## Generate changelog from git commits
	@echo "$(BLUE)Generating changelog...$(NC)"
	@./scripts/generate-changelog.sh

##@ Running

run: ## Run the application (debug mode)
	@echo "$(BLUE)Running application...$(NC)"
	@cargo run

run-release: ## Run the application (release mode)
	@echo "$(BLUE)Running release build...$(NC)"
	@cargo run --release

watch: ## Auto-rebuild and run on file changes
	@echo "$(BLUE)Watching for changes...$(NC)"
	@cargo watch -x run

watch-test: ## Auto-run tests on file changes
	@echo "$(BLUE)Watching tests...$(NC)"
	@cargo watch -x test

##@ Security & Dependencies

audit: ## Check for security vulnerabilities
	@echo "$(BLUE)Running security audit...$(NC)"
	@cargo audit
	@echo "$(GREEN)✓ No vulnerabilities found$(NC)"

outdated: ## Check for outdated dependencies
	@echo "$(BLUE)Checking for outdated dependencies...$(NC)"
	@cargo install cargo-outdated || true
	@cargo outdated
	@echo "$(GREEN)✓ Check complete$(NC)"

update: ## Update dependencies
	@echo "$(BLUE)Updating dependencies...$(NC)"
	@cargo update
	@echo "$(GREEN)✓ Dependencies updated$(NC)"

##@ Maintenance

install-tools: ## Install all development tools
	@echo "$(BLUE)Installing development tools...$(NC)"
	@cargo install mdbook git-cliff cargo-audit cargo-watch cargo-outdated || true
	@echo "$(GREEN)✓ Tools installed$(NC)"

verify: fmt-check lint test ## Verify code quality (CI simulation)
	@echo "$(GREEN)✓ All verifications passed$(NC)"

ci: verify build doc ## Run full CI checks locally
	@echo "$(GREEN)✓ CI checks complete$(NC)"

##@ Git Operations

commit: fmt lint ## Format, lint, then prepare to commit
	@echo "$(GREEN)✓ Code is ready to commit$(NC)"
	@echo "$(YELLOW)Run: git commit -m 'type: message'$(NC)"

push-check: pre-push ## Alias for pre-push checks
	@echo "$(GREEN)✓ Ready to push$(NC)"

##@ Development

dev: ## Start development environment (watch + serve docs)
	@echo "$(BLUE)Starting development environment...$(NC)"
	@echo "$(YELLOW)Starting documentation server...$(NC)"
	@mdbook serve &
	@echo "$(YELLOW)Starting cargo watch...$(NC)"
	@cargo watch -x run

example: ## Run an example (usage: make example NAME=basic_usage)
	@echo "$(BLUE)Running example: $(NAME)$(NC)"
	@cargo run --example $(NAME)

new-feature: ## Create a new feature branch (usage: make new-feature NAME=my-feature)
	@echo "$(BLUE)Creating feature branch: feature/$(NAME)$(NC)"
	@git checkout -b feature/$(NAME)
	@echo "$(GREEN)✓ Branch created and checked out$(NC)"

##@ Release

tag: ## Create a new version tag (usage: make tag VERSION=0.1.0)
	@echo "$(BLUE)Creating tag v$(VERSION)$(NC)"
	@git tag v$(VERSION)
	@echo "$(GREEN)✓ Tag created$(NC)"
	@echo "$(YELLOW)Push with: git push origin v$(VERSION)$(NC)"

release-prep: changelog ## Prepare for release (update changelog)
	@echo "$(GREEN)✓ Release preparation complete$(NC)"
	@echo "$(YELLOW)Next steps:$(NC)"
	@echo "  1. Review CHANGELOG.md"
	@echo "  2. Update version in Cargo.toml"
	@echo "  3. make tag VERSION=x.y.z"
	@echo "  4. git push origin vx.y.z"

##@ Python Bindings

python-build: ## Build Rust with Python feature enabled
	@echo "$(BLUE)Building with Python bindings...$(NC)"
	@cargo build --features python
	@echo "$(GREEN)✓ Python build complete$(NC)"

python-dev: ## Install Python package in development mode
	@echo "$(BLUE)Installing Python package (dev mode)...$(NC)"
	@maturin develop
	@echo "$(GREEN)✓ Python package installed$(NC)"

python-dev-release: ## Install Python package in release mode
	@echo "$(BLUE)Installing Python package (release mode)...$(NC)"
	@maturin develop --release
	@echo "$(GREEN)✓ Python package installed (release)$(NC)"

python-test: python-dev ## Run Python tests
	@echo "$(BLUE)Running Python tests...$(NC)"
	@pytest python/tests/ -v
	@echo "$(GREEN)✓ Python tests passed$(NC)"

python-stubs: ## Generate Python type stubs from Rust
	@echo "$(BLUE)Generating Python type stubs...$(NC)"
	@./scripts/generate_stubs.sh
	@echo "$(GREEN)✓ Type stubs generated$(NC)"

python-typecheck: ## Run mypy type checking on Python code
	@echo "$(BLUE)Running mypy type check...$(NC)"
	@mypy python/tests/
	@echo "$(GREEN)✓ Type check passed$(NC)"

python-lint: ## Lint Python code with ruff
	@echo "$(BLUE)Linting Python code...$(NC)"
	@ruff check python/
	@echo "$(GREEN)✓ Python lint passed$(NC)"

python-lint-fix: ## Auto-fix Python linting issues
	@echo "$(BLUE)Auto-fixing Python lint issues...$(NC)"
	@ruff check python/ --fix
	@echo "$(GREEN)✓ Python lint fixes applied$(NC)"

python-fmt: ## Format Python code with ruff
	@echo "$(BLUE)Formatting Python code...$(NC)"
	@ruff format python/
	@echo "$(GREEN)✓ Python code formatted$(NC)"

python-wheel: ## Build Python wheel for distribution
	@echo "$(BLUE)Building Python wheel...$(NC)"
	@maturin build --release
	@echo "$(GREEN)✓ Wheel built in target/wheels/$(NC)"

python-wheel-all: ## Build wheels for all Python versions
	@echo "$(BLUE)Building wheels for all Python versions...$(NC)"
	@maturin build --release --find-interpreter
	@echo "$(GREEN)✓ All wheels built$(NC)"

python-clean: ## Clean Python build artifacts
	@echo "$(BLUE)Cleaning Python artifacts...$(NC)"
	@rm -rf python/synapse_db/*.so python/synapse_db/*.pyd
	@rm -rf python/synapse_db/__pycache__ python/tests/__pycache__
	@rm -rf .pytest_cache .mypy_cache .ruff_cache
	@rm -rf dist/ *.egg-info/
	@echo "$(GREEN)✓ Python clean complete$(NC)"

python-all: python-stubs python-dev python-lint python-typecheck python-test ## Run all Python checks
	@echo "$(GREEN)✓ All Python checks passed$(NC)"

python-ci: python-stubs python-dev python-fmt python-lint python-typecheck python-test python-wheel ## Full Python CI pipeline
	@echo "$(GREEN)✓ Python CI complete$(NC)"

python-install-deps: ## Install Python development dependencies
	@echo "$(BLUE)Installing Python dependencies...$(NC)"
	@pip install maturin pytest pytest-cov mypy ruff
	@echo "$(GREEN)✓ Python dependencies installed$(NC)"

##@ Information

info: ## Show project information
	@echo "$(BLUE)Project Information$(NC)"
	@echo "Name:        Synapse DB"
	@echo "Language:    Rust + Python"
	@echo "Version:     $$(grep '^version' Cargo.toml | head -1 | cut -d'"' -f2)"
	@echo ""
	@echo "$(BLUE)Rust Toolchain$(NC)"
	@echo "Rust:        $$(rustc --version)"
	@echo "Cargo:       $$(cargo --version)"
	@echo ""
	@echo "$(BLUE)Python Toolchain$(NC)"
	@echo "Python:      $$(python --version 2>/dev/null || echo 'not found')"
	@echo "Maturin:     $$(maturin --version 2>/dev/null || echo 'not found')"
	@echo "Pytest:      $$(pytest --version 2>/dev/null | head -1 || echo 'not found')"
	@echo ""
	@echo "$(BLUE)Project Structure$(NC)"
	@echo "Rust src:    src/"
	@echo "Rust tests:  tests/"
	@echo "Python pkg:  python/synapse_db/"
	@echo "Python tests: python/tests/"
	@echo "Docs:        book/"
	@echo "Scripts:     scripts/"

status: ## Show git status
	@git status

list-targets: ## List all make targets
	@$(MAKE) -qp | awk -F':' '/^[a-zA-Z0-9][^$$#\/\t=]*:([^=]|$$)/ {split($$1,A,/ /);for(i in A)print A[i]}'
