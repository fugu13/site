.PHONY: help build serve dev lint fmt audit clean

help: ## Show available targets
	@grep -hE '^[a-zA-Z_-]+:.*## ' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*## "}; {printf "%-10s %s\n", $$1, $$2}'

build: ## Prerender the site to dist/ (the deployable artifact)
	cargo run --release --bin prerender

serve: ## Serve the prerendered site as a static host would
	cargo run --release --bin serve

dev: build ## Prerender, then serve; re-run to pick up changes
	$(MAKE) serve

lint: audit ## Run clippy (deny warnings), rustfmt check, and the security-advisory audit
	cargo clippy --all-targets -- -D warnings
	cargo fmt -- --check

fmt: ## Auto-format
	cargo fmt

audit: ## Check Cargo.lock against the RustSec advisory database (requires cargo-audit)
	cargo audit

clean: ## Remove build artifacts
	rm -rf target dist
