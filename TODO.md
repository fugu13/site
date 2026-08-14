# TODO

Items use stable identifiers with a prefix matching their subject area. Group related TODOs under a header; completed items move to the end of their section.

## Accessibility

### A11Y-001: Home page heading hierarchy skips h2

The home page's heading hierarchy jumps from the page's `<h1>` directly to an `<h3>` for the "Blog" section heading, with no `<h2>` in between. This was not fixed during the Perseus-to-Leptos migration: sakura.css gives `<h2>` and `<h3>` different default sizes, so changing the heading level would visibly change the size of the "Blog" heading — and this migration was explicitly scoped to preserve the page's current layout, not to alter it.

### A11Y-002: Long code blocks have no scrollable, keyboard-focusable region

Long code blocks render as bare `<pre><code>` with no wrapping `role="region" tabindex="0"` container, so a horizontally overflowing block can't be scrolled into view by a keyboard-only user the way the equivalent code blocks on this author's other project, pgdmn, can. This was not added during the migration because it would require new wrapping DOM structure around the existing `<pre><code>` markup — again beyond this migration's scope of preserving the existing DOM shape.
