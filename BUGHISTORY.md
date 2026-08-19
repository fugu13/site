# Bug History

## Malformed internal links in article Markdown (two occurrences)

**Symptom:** An in-article link to another post 404s. Two shipped instances: `articles/property-based-testing-from-scratch.md` linked `/post//muddled-property-based-tests/` (double slash), and `articles/why-webassembly.md` linked `post/static-site-rust-perseus/` (no leading slash, so the browser resolved it relative to the current post's directory as `/post/why-webassembly/post/static-site-rust-perseus/`).

**Root cause:** Internal links in article Markdown are free-form text with no validation anywhere in the build — a typo'd path renders fine and only fails when a reader or crawler follows it. The relative-link variant is the more insidious one because it looks almost correct in the source.

**Fix:** Corrected both links to the absolute `/post/<slug>/` form. No automated guard exists yet; LINK-001 in TODO.md tracks adding an internal link check at build or CI time.

**Files involved:** `articles/property-based-testing-from-scratch.md`, `articles/why-webassembly.md`.

**Reoccurrence check:** Every link to another post inside `articles/*.md` must start with `/post/` — absolute from the site root, exactly one slash after `post`, trailing slash at the end. When editing any article, check its internal links against the pattern; until LINK-001 lands, a quick `grep -n '](post/' articles/*.md` and `grep -n '/post//' articles/*.md` should both return nothing.

## Two words glued together on four live posts (not reproduced on the new site)

**Symptom:** On the live (pre-migration) site, four posts each have exactly one spot where two words run together with no space: "haven't interacted with it much" on `/post/why-webassembly/` renders as "haven't interactedwith it much"; "traversal for a binary tree" on `/post/testing-data-structures-binary-tree/` renders as "traversal for abinary tree"; "PL/Rust, you can use" on `/post/beautiful-postgresql-templates/` renders as "PL/Rust,you can use"; and "I'm using one called Perseus" on `/post/static-site-rust-perseus/` renders as "I'm usingone called Perseus". A word-by-word diff of each full article body against the live page found exactly one such discrepancy per page.

**Root cause:** A bug in the old (pre-migration) Perseus/`markdown`-crate rendering pipeline, not in the new site. Each affected `.md` source hard-wraps its opening paragraph at the exact word pair that's glued on the live site; a byte-exact check of the live page's precomputed HTML shows the legacy renderer dropped the first soft-line-break newline of each post's body outright (plausibly a `str::replace`-first-occurrence meant to trim a leading blank line but applied to the whole rendered string), while every other soft break in the same articles renders correctly as a space on the live site.

**Fix:** None needed in the new site — `crate::markdown::to_html` (`src/markdown.rs`) renders soft breaks as CommonMark specifies (a normal space), so all four spots render correctly by default on the migrated site. An earlier pass mistakenly added a `patch_legacy_content_quirks` function to `src/pages/post.rs` that re-glued these four word pairs back together in the new site's output, on the reasoning that migration should preserve "the same final DOM." That reasoning doesn't apply here: this is a content-correctness bug in the retired renderer, not a matter of visual layout, styling, or images (the things this migration was scoped to preserve) — reproducing it would mean shipping four known typos on purpose. The patch was removed.

**Files involved:** `src/markdown.rs`, `src/pages/post.rs`.

**Reoccurrence check:** `src/pages/post.rs` should not contain a `patch_legacy_content_quirks` function or any other per-slug string-patching of rendered post HTML. If a future content discrepancy against the old live site is found, first determine whether the *old* site or the *new* site has the defect — only patch the new site to match old behavior if the old behavior was actually correct.
