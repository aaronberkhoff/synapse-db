# Synapse DB Makefile
# Provides convenient commands for development tasks

.PHONY: help setup build test clean fmt lint doc check run release install-hooks pre-push changelog serve watch audit outdated all

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

all: fmt lint test build doc ## Run all checks and build

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

##@ Information

info: ## Show project information
	@echo "$(BLUE)Project Information$(NC)"
	@echo "Name:        Synapse DB"
	@echo "Language:    Rust"
	@echo "Version:     $$(grep '^version' Cargo.toml | head -1 | cut -d'"' -f2)"
	@echo "Rust:        $$(rustc --version)"
	@echo "Cargo:       $$(cargo --version)"
	@echo ""
	@echo "$(BLUE)Project Structure$(NC)"
	@echo "Source:      src/"
	@echo "Tests:       tests/"
	@echo "Docs:        book/"
	@echo "Scripts:     scripts/"
	@echo "Examples:    examples/"

status: ## Show git status
	@git status

list-targets: ## List all make targets
	@$(MAKE) -qp | awk -F':' '/^[a-zA-Z0-9][^$$#\/\t=]*:([^=]|$$)/ {split($$1,A,/ /);for(i in A)print A[i]}'
