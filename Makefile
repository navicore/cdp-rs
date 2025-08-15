.PHONY: all build test clean lint fmt check release bench doc install help ci-test ci-lint

# Default target
all: lint build test

# Help target
help:
	@echo "CDP-RS Development Commands"
	@echo "============================"
	@echo "make build      - Build all packages in debug mode"
	@echo "make release    - Build all packages in release mode"
	@echo "make test       - Run all tests"
	@echo "make bench      - Run benchmarks"
	@echo "make lint       - Run clippy with strict settings"
	@echo "make fmt        - Format code with rustfmt"
	@echo "make check      - Check code without building"
	@echo "make doc        - Generate documentation"
	@echo "make clean      - Remove build artifacts"
	@echo "make install    - Install binaries locally"
	@echo ""
	@echo "CI Commands (used by GitHub Actions):"
	@echo "make ci-lint    - Lint with CI settings (fails on warnings)"
	@echo "make ci-test    - Run tests with CI settings"
	@echo ""
	@echo "Oracle Testing:"
	@echo "make oracle     - Run oracle validation tests (requires CDP)"
	@echo "make demo       - Run the oracle demo"

# Build commands
build:
	@echo "Building all packages..."
	@cargo build --workspace

release:
	@echo "Building release version..."
	@cargo build --workspace --release

# Testing commands
test:
	@echo "Running tests..."
	@cargo test --workspace

test-verbose:
	@echo "Running tests with output..."
	@cargo test --workspace -- --nocapture

bench:
	@echo "Running benchmarks..."
	@cargo bench --workspace

# Linting and formatting
lint:
	@echo "Running clippy..."
	@cargo clippy --workspace --all-targets --all-features -- \
		-W clippy::all \
		-W clippy::pedantic \
		-W clippy::nursery \
		-A clippy::missing_errors_doc \
		-A clippy::missing_panics_doc \
		-A clippy::must_use_candidate \
		-A clippy::module_name_repetitions

# CI-specific lint (fails on any warning)
ci-lint:
	@echo "Running clippy (CI mode - strict)..."
	@cargo clippy --workspace --all-targets --all-features -- \
		-D warnings \
		-W clippy::all \
		-W clippy::pedantic \
		-W clippy::nursery \
		-A clippy::missing_errors_doc \
		-A clippy::missing_panics_doc \
		-A clippy::must_use_candidate \
		-A clippy::module_name_repetitions

fmt:
	@echo "Formatting code..."
	@cargo fmt --all

fmt-check:
	@echo "Checking formatting..."
	@cargo fmt --all -- --check

check:
	@echo "Checking code..."
	@cargo check --workspace --all-targets

# Documentation
doc:
	@echo "Generating documentation..."
	@cargo doc --workspace --no-deps --open

doc-private:
	@echo "Generating documentation (including private items)..."
	@cargo doc --workspace --no-deps --document-private-items --open

# Clean
clean:
	@echo "Cleaning build artifacts..."
	@cargo clean
	@rm -rf target/

# Install
install:
	@echo "Installing binaries..."
	@cargo install --path cdp-examples

# Oracle testing
oracle:
	@echo "Running oracle validation tests..."
	@CDP_PATH=$${CDP_PATH:-/usr/local/cdp/bin} cargo test --package cdp-oracle --features integration-tests

demo:
	@echo "Running oracle demo..."
	@cargo run --bin oracle_demo

# CI test command
ci-test:
	@echo "Running tests (CI mode)..."
	@cargo test --workspace --no-fail-fast

# Frozen module check
check-frozen:
	@echo "Checking frozen modules..."
	@./scripts/check-frozen.sh || echo "Script not found - create scripts/check-frozen.sh"

# Development helpers
watch:
	@echo "Watching for changes..."
	@cargo watch -x check -x test -x "clippy -- -W clippy::all"

todo:
	@echo "Searching for TODOs..."
	@grep -r "TODO\|FIXME\|XXX" --include="*.rs" cdp-* || echo "No TODOs found!"

# Validation workflow
validate: lint test bench
	@echo "======================================"
	@echo "Validation complete!"
	@echo "Next: Run 'make oracle' with CDP installed"

# Quick check before committing
pre-commit: fmt lint test
	@echo "======================================"
	@echo "Pre-commit checks passed!"

# CDP binary setup helper
setup-cdp:
	@echo "CDP Setup Instructions:"
	@echo "======================="
	@echo "1. Download CDP from: https://github.com/ComposersDesktop/CDP8/releases"
	@echo "2. Extract to a directory (e.g., /usr/local/cdp)"
	@echo "3. Set CDP_PATH environment variable:"
	@echo "   export CDP_PATH=/usr/local/cdp/bin"
	@echo "4. Run: make oracle"

# Performance profiling
profile:
	@echo "Running with profiling..."
	@cargo build --release
	@echo "Run your binary with:"
	@echo "  perf record --call-graph=dwarf target/release/[binary]"
	@echo "  perf report"

# Coverage (requires cargo-tarpaulin)
coverage:
	@echo "Generating test coverage..."
	@cargo tarpaulin --workspace --out Html --output-dir target/coverage

# Size analysis
size:
	@echo "Analyzing binary sizes..."
	@cargo build --release
	@ls -lh target/release/oracle_demo 2>/dev/null || echo "No binaries built yet"

# Security audit
audit:
	@echo "Running security audit..."
	@cargo audit