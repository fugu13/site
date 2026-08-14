# CLAUDE.md

## Stack

- Rust, edition 2024. Single crate — the `[workspace]` table in `Cargo.toml` is empty on purpose, to stop Cargo's workspace-root search from walking up into the parent checkout when this repo is used as a worktree.
- Leptos 0.8 (`ssr` feature only), used purely as a build-time HTML templating engine. No hydration, no WebAssembly ships to production.
- Axum + Tokio: used only by the `serve` binary, to preview the prerendered `dist/` output locally the way a real static host would.
- `pulldown-cmark` (Markdown), `syntect` (build-time syntax highlighting), `grass` (Sass → CSS), `include_dir` (embeds `articles/` into the binary), `serde` / `serde_yaml` / `chrono` (front matter).

## Build & Run

`make` targets are the sanctioned way to build, serve, lint, and format this project — do not invoke `cargo build`, `cargo run`, `cargo clippy`, or `cargo fmt` directly for these.

| Command | Action |
|---|---|
| `make build` | Prerenders every route to `dist/` — the deployable artifact |
| `make serve` | Serves `dist/` the way the production static host does, including a real HTTP 404 status |
| `make dev` | `build`, then `serve`; re-run to pick up edits |
| `make lint` | `cargo clippy --all-targets -- -D warnings`, then `cargo fmt -- --check` |
| `make fmt` | `cargo fmt` |
| `make clean` | Removes `target/` and `dist/` |

CI (`.github/workflows/deploy.yml`) runs `make lint` then `make build` on every push and pull request, then independently re-verifies `dist/` (see Conventions).

## Content Model

A blog post is a file in `articles/*.md`: YAML front matter (`title`, `date`, optional `description` and `image`) followed by a `---`-delimited Markdown body. **Adding a post requires no code changes** — drop a new file into `articles/` matching the existing front matter shape, and `make build` picks it up. The slug is the filename stem.

See `docs/architecture.md` for the full build pipeline and route structure.

## Conventions

- **No client-side JavaScript except three named analytics domains.** The prerender step strips every `<script>` tag from its output except the three that load `tinyanalytics.io`, `simpleanalyticscdn.com`, and `plausible.io` — this also removes Leptos's own inert hydration bookkeeping script, since the site never hydrates in the browser. CI independently re-checks `dist/` for the same three domains and fails the build on anything else, and on any `.wasm` reference. Don't add a fourth script tag, and don't work around the check — if a new script is genuinely needed, the convention and the CI check change together, deliberately.
- **Markdown rendering is deliberately CommonMark-only.** `Options::empty()` in `src/markdown.rs` disables tables, strikethrough, and smart punctuation on purpose, so straight quotes in source posts are never rewritten to curly ones. Don't enable `pulldown-cmark` extensions without checking whether existing posts depend on the current behavior.
- **Fenced code blocks stay bare `<pre><code>`.** No wrapping `<div>`, no `role`/`tabindex`. See `TODO.md` (A11Y-002) for the accessibility gap this leaves and why it wasn't closed during the migration.
- **Accessibility is a default, not something weighed case by case.** New UI work should not introduce accessibility gaps even though two pre-existing ones (`TODO.md`, A11Y-001 and A11Y-002) are tracked as deliberately out of scope for the Perseus-to-Leptos migration — that migration was scoped to preserve the site's existing DOM shape and layout, not to improve or regress it.
- **Error handling:** no `unwrap()`, `expect()`, or `panic!()` outside test code for data that could plausibly be malformed. The exception is a genuinely unrecoverable build/dev-tool condition, where panicking immediately with a clear message beats threading a `Result` through every call site — e.g. `posts::all` on a malformed article, `assets::set_stylesheet_path` on being read before it's set, and `serve`'s local preview server failing to bind its port. Any new panic in non-test code needs the same bar, and should be documented at the call site the way these are.

## Testing

Unit tests live in `#[cfg(test)]` blocks alongside the code they test (`src/posts.rs`, `src/markdown.rs`, `src/highlight.rs`), asserting on rendered HTML fragments. Run them with `cargo test` — there is no `make test` target yet, so this is the one case where invoking `cargo` directly is expected rather than routed through `make`. New front-matter, Markdown, or highlighting logic should get unit tests in the same module, following the existing pattern.

## Documentation

| File | Purpose |
|---|---|
| `README.md` | Project description, doc index |
| `TODO.md` | Tracked work items |
| `BUGHISTORY.md` | Resolved bugs with reoccurrence checks |
| `docs/architecture.md` | Site-generation architecture |
