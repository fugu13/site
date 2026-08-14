# TODO

Items use stable identifiers with a prefix matching their subject area. Group related TODOs under a header; completed items move to the end of their section.

## Accessibility

### A11Y-001: Home page heading hierarchy skips h2 (Implemented 2026-08-14)

The home page's heading hierarchy jumped from the page's `<h1>` directly to an `<h3>` for the "Blog" section heading, with no `<h2>` in between. Fixed by changing the "Blog" heading to `<h2 class="h3-size">`, a new stylesheet class (`style/main.scss`) matching the original `<h3>` size so the visible heading is unchanged.

### A11Y-002: Long code blocks have no scrollable, keyboard-focusable region (Implemented 2026-08-14)

Long code blocks rendered as bare `<pre><code>` with no wrapping `role="region" tabindex="0"` container, so a horizontally overflowing block couldn't be scrolled into view by a keyboard-only user the way the equivalent code blocks on this author's other project, pgdmn, can. Fixed in `src/markdown.rs` by wrapping every fenced code block's `<pre><code>` in a `<div role="region" tabindex="0" aria-label="Code sample">`.
