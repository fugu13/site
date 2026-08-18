PORT := 4000
URL := http://127.0.0.1:$(PORT)

.PHONY: help build serve dev open blog draft lint fmt audit clean

help: ## Show available targets
	@grep -hE '^[a-zA-Z_-]+:.*## ' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*## "}; {printf "%-10s %s\n", $$1, $$2}'

build: ## Prerender the site to dist/ (the deployable artifact)
	cargo run --release --bin prerender

serve: ## Serve the prerendered site as a static host would
	cargo run --release --bin serve

dev: build ## Prerender, then serve; re-run to pick up changes
	$(MAKE) serve

# Not listed in `make help`: an internal piece reused by targets that preview a
# running `make serve`. Keep $(PORT) matching the port hardcoded in src/bin/serve.rs.
open:
	open $(URL)

blog: ## Commit added/updated posts under articles/ and public/ to a new branch, push it, then preview locally
	scripts/blog-commit.sh
	$(MAKE) build
	( n=0; until curl -sf $(URL) >/dev/null 2>&1 || [ $$n -ge 40 ]; do sleep 0.5; n=$$((n + 1)); done; $(MAKE) open ) &
	$(MAKE) serve

draft: ## Create articles/draft.md, a placeholder draft post ready to fill in
	scripts/new-draft.sh

lint: audit ## Run clippy (deny warnings), rustfmt check, and the security-advisory audit
	cargo clippy --all-targets -- -D warnings
	cargo fmt -- --check

fmt: ## Auto-format
	cargo fmt

audit: ## Check Cargo.lock against the RustSec advisory database (requires cargo-audit)
	cargo audit

clean: ## Remove build artifacts
	rm -rf target dist
