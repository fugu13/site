# Russell Duhon's Blog

Source for https://www.russellduhon.com — a personal blog: a home page listing posts and one page per post, generated from Markdown files in `articles/`.

The site is built by a from-scratch, Leptos-0.8-ssr-only static generator: Leptos is used purely as a build-time HTML templating engine (no hydration, no WebAssembly ships to production). `make build` prerenders every route to plain static HTML and CSS in `dist/`, which a GitHub Actions workflow deploys to GitHub Pages.

## Build & Run

| Command | Action |
|---|---|
| `make build` | Prerender the site to `dist/` — the deployable artifact |
| `make serve` | Serve the prerendered `dist/` the way a static host would |
| `make dev` | `build`, then `serve`; re-run to pick up changes |
| `make lint` | Run clippy (deny warnings), check formatting, and audit dependencies |
| `make fmt` | Auto-format the code |
| `make audit` | Check `Cargo.lock` against the RustSec advisory database |
| `make draft` | Scaffold `articles/draft.md`, a placeholder draft post |
| `make blog` | Commit new/updated posts to a new branch, push it, then build and preview locally |

## Documentation

| File | Purpose |
|---|---|
| [CLAUDE.md](CLAUDE.md) | Development conventions |
| [TODO.md](TODO.md) | Tracked work items |
| [BUGHISTORY.md](BUGHISTORY.md) | Resolved bugs with reoccurrence checks |
| [docs/architecture.md](docs/architecture.md) | Site-generation architecture |
| [docs/seo.md](docs/seo.md) | Search indexing and syndication: sitemap, feed, metadata, structured data |
| [docs/ux/code-blocks.md](docs/ux/code-blocks.md) | Keyboard behavior for scrolling wide code samples |
| [docs/ux/heading-structure.md](docs/ux/heading-structure.md) | Heading hierarchy for screen-reader navigation |
| [docs/ux/page-titles.md](docs/ux/page-titles.md) | Browser-tab and search-result titles per page |
