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
| `make lint` | `make audit`, then `cargo clippy --all-targets -- -D warnings`, then `cargo fmt -- --check` |
| `make fmt` | `cargo fmt` |
| `make audit` | `cargo audit` — fails only on an actual RustSec vulnerability advisory, not on "unmaintained" warnings |
| `make draft` | Creates `articles/draft.md`, a timestamped placeholder draft post; refuses to overwrite an existing one |
| `make blog` | Commits added/updated files under `articles/` and `public/` to a new branch (message auto-generated from which posts changed), pushes it, then builds and previews the site locally. Requires `main` to be checked out and up to date with `origin/main`, so the new branch never carries along unrelated commits |
| `make clean` | Removes `target/` and `dist/` |

CI (`.github/workflows/deploy.yml`) runs `make lint` then `make build` on every push and pull request, then independently re-verifies `dist/` (see Conventions).

## Content Model

A blog post is a file in `articles/*.md`: YAML front matter (`title`, `date`, optional `updated`, `description`, `image`, and `draft`) followed by a `---`-delimited Markdown body. `updated` records when a post was last substantively edited and feeds the sitemap's `lastmod`, the feed's `updated`, and the structured data's `dateModified`; set it when meaningfully revising a published post. A fenced code block's info string is the language optionally followed by a short label — like ```` ```python The complete test file ```` — and the label becomes the block's screen-reader description, so write one for every fence. **Adding a post requires no code changes** — drop a new file into `articles/` matching the existing front matter shape, and `make build` picks it up. The slug is the filename stem. A post with `draft: true` is skipped entirely at build time — no route, no home page listing — until the flag is removed; `make draft` scaffolds one, and `make blog` notes draft status in its generated commit message.

See `docs/architecture.md` for the full build pipeline and route structure.

## Conventions

- **No client-side JavaScript except three named analytics domains.** The prerender step strips every `<script>` tag from its output except the three that load `tinyanalytics.io`, `simpleanalyticscdn.com`, and `plausible.io` — this also removes Leptos's own inert hydration bookkeeping script, since the site never hydrates in the browser. The one non-loading exception is `<script type="application/ld+json">` data blocks: inert JSON-LD structured data for search engines, never executed, kept by the strip step and allowed by CI. CI independently re-checks `dist/` for the three domains plus the JSON-LD form and fails the build on anything else, and on any `.wasm` reference. Don't add a fourth script tag, and don't work around the check — if a new script is genuinely needed, the convention and the CI check change together, deliberately.
- **Markdown rendering is deliberately CommonMark-only.** `Options::empty()` in `src/markdown.rs` disables tables, strikethrough, and smart punctuation on purpose, so straight quotes in source posts are never rewritten to curly ones. Don't enable `pulldown-cmark` extensions without checking whether existing posts depend on the current behavior.
- **Fenced code blocks are wrapped in a focusable scroll region.** `src/markdown.rs` wraps every `<pre><code>` in `<div role="region" tabindex="0">` whose `aria-label` is the fence's label from the info string ("Code sample" when none is written — see Content Model), so a keyboard-only user can scroll a horizontally overflowing block into view and a screen-reader user hears which sample it is. Don't add a bare, unwrapped `<pre><code>` for new code output — go through `crate::markdown::to_html` (or match its wrapping) instead.
- **Heading levels follow the document outline, never the wanted text size.** Size comes from a class (`.h3-size`, `.h4-size` in `style/main.scss`) on the outline-correct element, and things that aren't section headings — navigation links, dates — aren't heading elements at all (`.post-nav`, `.post-date` keep their look). Article Markdown starts its headings at `##`, one level below the page's `<h1>`, with no skipped levels.
- **Accessibility is a default, not something weighed case by case.** New UI work should not introduce accessibility gaps.
- **Error handling:** no `unwrap()`, `expect()`, or `panic!()` outside test code for data that could plausibly be malformed. The exception is a genuinely unrecoverable build/dev-tool condition, where panicking immediately with a clear message beats threading a `Result` through every call site — e.g. `posts::all` on a malformed article, `assets::set_stylesheet_path` on being read before it's set, and `serve`'s local preview server failing to bind its port. Any new panic in non-test code needs the same bar, and should be documented at the call site the way these are.
- **Lint baseline: clippy `pedantic` + `nursery`, both warn-level, `-D warnings` in CI** (`Cargo.toml`'s `[lints.clippy]`). A handful of lints are allowed project-wide, each with a one-line reason in `Cargo.toml`; a lint that fires somewhere that reason doesn't cover should get fixed there, not silenced with a new blanket allow. A genuinely one-off false positive gets a local `#[allow(..., reason = "...")]` at the call site instead of a project-wide entry.

## Repository Practices

`main` is a protected branch on GitHub: pushes go through a pull request, the `build` CI check (lint, build, `dist/` verification) must pass before merging, and this is enforced for admins too — there's no direct-push exception, including for the repo owner. Force-pushes and deletion of `main` are disabled. Merged branches are deleted automatically. Dependabot opens weekly, grouped update PRs for both Cargo and GitHub Actions dependencies, and separately raises PRs for any known security advisory; secret scanning and push protection are on. None of this is configured in-repo (it's GitHub repository/branch settings, not a file) — noted here so it doesn't get rediscovered by surprise.

## Testing

Unit tests live in `#[cfg(test)]` blocks alongside the code they test (`src/posts.rs`, `src/markdown.rs`, `src/highlight.rs`), asserting on rendered HTML fragments. Run them with `cargo test` — there is no `make test` target yet, so this is the one case where invoking `cargo` directly is expected rather than routed through `make`. New front-matter, Markdown, or highlighting logic should get unit tests in the same module, following the existing pattern.

## Documentation

| File | Purpose |
|---|---|
| `README.md` | Project description, doc index |
| `TODO.md` | Tracked work items |
| `BUGHISTORY.md` | Resolved bugs with reoccurrence checks |
| `docs/architecture.md` | Site-generation architecture |
