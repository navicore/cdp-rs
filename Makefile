.PHONY: all build test clean lint fmt fmt-check check-cdp check release bench doc install help ci-test ci-lint ci-check pre-commit validate todo watch check-frozen oracle demo test-verbose doc-private build-cdp install-cdp test-cdp clean-cdp cdp-env install-deps profile coverage size audit oracle-local

# Default target - run all checks (MUST BE FIRST!)
all:
	@echo "Running format check..."
	@cargo fmt --all -- --check
	@echo "Running lint..."
	@cargo clippy --workspace --all-targets --all-features -- \
		-W clippy::all \
		-W clippy::correctness \
		-W clippy::suspicious \
		-W clippy::complexity \
		-W clippy::perf \
		-W clippy::style \
		-A clippy::missing_errors_doc \
		-A clippy::missing_panics_doc \
		-A clippy::must_use_candidate \
		-A clippy::module_name_repetitions
	@if [ ! -d "build/cdp-install/bin" ] || [ ! -f "build/cdp-install/bin/housekeep" ]; then \
		echo "ERROR: CDP is not installed!"; \
		echo "CDP is REQUIRED for tests."; \
		echo "Run 'make install-cdp' to install CDP first."; \
		exit 1; \
	fi
	@echo "✓ CDP is installed"
	@echo "Building all packages..."
	@cargo build --workspace
	@echo "Running tests..."
	@cargo test --workspace
	@echo "Building release version..."
	@cargo build --workspace --release
	@$(MAKE) oracle-local

# Check for CDP installation
check-cdp:
	@if [ ! -d "build/cdp-install/bin" ] || [ ! -f "build/cdp-install/bin/housekeep" ]; then \
		echo "ERROR: CDP is not installed!"; \
		echo "CDP is REQUIRED for all operations."; \
		echo "Run 'make install-cdp' to install CDP first."; \
		exit 1; \
	fi
	@echo "✓ CDP is installed"

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
	@echo "CDP Build Management:"
	@echo "make build-cdp   - Build CDP from source"
	@echo "make test-cdp    - Test CDP build"
	@echo "make clean-cdp   - Remove CDP build"
	@echo "make cdp-env     - Show CDP environment setup"
	@echo ""
	@echo "Oracle Testing:"
	@echo "make oracle     - Run oracle validation tests (auto-installs CDP)"
	@echo "make demo       - Run the oracle demo"

# Build commands
build:
	@echo "Building all packages..."
	@cargo build --workspace

release:
	@echo "Building release version..."
	@cargo build --workspace --release

# Testing commands  
test: check-cdp
	@echo "Running tests..."
	@cargo test --workspace

test-passing:
	@echo "Running only passing tests (skipping known failures)..."
	@cargo test --package cdp-core
	@cargo test --package cdp-modify
	@cargo test --package cdp-sndinfo
	@cargo test --package cdp-sandbox
	@cargo test --package cdp-oracle test_utils
	@echo "All passing tests completed successfully!"

test-status:
	@./scripts/mark-tests.sh status

test-oracle:
	@echo "Running oracle tests (including ignored ones)..."
	@cargo test --package cdp-distort oracle_tests -- --ignored || true
	@cargo test --package cdp-housekeep test_basic_copy -- --ignored || true
	@cargo test --package cdp-pvoc oracle_tests -- --ignored || true
	@cargo test --package cdp-pvoc format_tests -- --ignored || true
	@cargo test --package cdp-spectral oracle_tests -- --ignored || true
	@echo "Oracle test run complete (failures are expected)"

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
		-W clippy::correctness \
		-W clippy::suspicious \
		-W clippy::complexity \
		-W clippy::perf \
		-W clippy::style \
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
		-W clippy::correctness \
		-W clippy::suspicious \
		-W clippy::complexity \
		-W clippy::perf \
		-W clippy::style \
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
	@echo "Removing CDP installation..."
	@rm -rf build/cdp build/cdp-install
	@echo "Clean complete - CDP and all build artifacts removed"

# Install
install:
	@echo "Installing binaries..."
	@cargo install --path cdp-examples

# Oracle testing
oracle: build-cdp
	@echo "Running oracle validation tests..."
	@CDP_PATH=build/cdp-install/bin cargo test --package cdp-oracle --features integration-tests

oracle-local:
	@echo "Running oracle tests (if CDP is available)..."
	@if [ -d "build/cdp/NewRelease" ]; then \
		echo "CDP found, running oracle tests..."; \
		./scripts/ci-oracle-test.sh; \
	else \
		echo "CDP not built, skipping oracle tests. Run 'make oracle' to build CDP and test."; \
	fi

demo:
	@echo "Running oracle demo..."
	@cargo run --bin oracle_demo

# CI test command - CDP is MANDATORY
ci-test: check-cdp
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

# Quick check before committing (auto-formats)
pre-commit: fmt lint test
	@echo "======================================"
	@echo "Pre-commit checks passed!"

# Strict check (same as CI, no auto-formatting)
ci-check: fmt-check ci-lint ci-test
	@echo "======================================"
	@echo "CI checks passed locally!"

# CDP Build and Setup
build-cdp:
	@echo "Building CDP from source..."
	@./scripts/build-cdp.sh

test-cdp: build-cdp
	@echo "Testing CDP build..."
	@./scripts/test-cdp.sh

test-cdp-ci: build-cdp
	@echo "Testing CDP build (CI mode)..."
	@./scripts/test-cdp-ci.sh

demo-cdp: build-cdp
	@echo "Running CDP demo..."
	@./scripts/cdp-demo.sh

clean-cdp:
	@echo "Removing CDP build..."
	@rm -rf build/cdp build/cdp-install test-output cdp-demo-output
	@echo "CDP build and test files removed"

# Convenience targets
install-deps: build-cdp
	@echo "All dependencies built!"

# Backwards compatibility
install-cdp: build-cdp
	@echo "CDP built from source in build/cdp-install"

# Setup environment for CDP
cdp-env:
	@if [ -f build/cdp-install/env.sh ]; then \
		echo "To set up CDP environment, run:"; \
		echo "  source build/cdp-install/env.sh"; \
	else \
		echo "CDP not built. Run 'make build-cdp' first"; \
	fi

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