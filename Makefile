# Copyright (c) 2025 TRUSTEDGE LABS LLC
# MPL-2.0: https://mozilla.org/MPL/2.0/
# Project: trustedge — Privacy and trust at the edge.

.PHONY: help copyright-check fix-copyright install-hooks build test clean

help: ## Show this help message
	@echo "TrustEdge Development Commands:"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}'

copyright-check: ## Check all files for copyright headers
	@./scripts/fix-copyright.sh

fix-copyright: ## Add copyright headers to files that are missing them
	@./scripts/fix-copyright.sh

install-hooks: ## Install git pre-commit hooks
	@echo "Installing pre-commit hook..."
	@cp scripts/pre-commit.sh .git/hooks/pre-commit
	@chmod +x .git/hooks/pre-commit
	@echo "✅ Pre-commit hook installed"

build: ## Build the project
	@cd trustedge-core && cargo build --release

build-with-audio: ## Build the project with audio features
	@cd trustedge-core && cargo build --release --features audio

test: ## Run all tests
	@cd trustedge-core && cargo test

test-with-audio: ## Run tests with audio features
	@cd trustedge-core && cargo test --features audio

clippy: ## Run clippy linting
	@cd trustedge-core && cargo clippy -- -D warnings

fmt: ## Format code
	@cd trustedge-core && cargo fmt

fmt-check: ## Check code formatting
	@cd trustedge-core && cargo fmt --check

audit: ## Run security audit
	@cd trustedge-core && cargo audit

clean: ## Clean build artifacts
	@cd trustedge-core && cargo clean

full-check: copyright-check clippy fmt-check test audit ## Run all quality checks

ci-check: ## Run the same checks as CI
	@echo "Running CI checks..."
	@$(MAKE) copyright-check
	@$(MAKE) fmt-check
	@$(MAKE) clippy
	@$(MAKE) test
	@$(MAKE) audit
	@echo "✅ All CI checks passed"

dev-setup: install-hooks ## Set up development environment
	@echo "Setting up development environment..."
	@cd trustedge-core && rustup component add clippy rustfmt
	@cd trustedge-core && cargo install cargo-audit || true
	@$(MAKE) install-hooks
	@echo "✅ Development environment ready"
