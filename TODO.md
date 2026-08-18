# TODO

Items use stable identifiers with a prefix matching their subject area. Group related TODOs under a header; completed items move to the end of their section.

## Accessibility

### A11Y-001: Home page heading hierarchy skips h2 (Implemented 2026-08-14)

The home page's heading hierarchy jumped from the page's `<h1>` directly to an `<h3>` for the "Blog" section heading, with no `<h2>` in between. Fixed by changing the "Blog" heading to `<h2 class="h3-size">`, a new stylesheet class (`style/main.scss`) matching the original `<h3>` size so the visible heading is unchanged.

### A11Y-002: Long code blocks have no scrollable, keyboard-focusable region (Implemented 2026-08-14)

Long code blocks rendered as bare `<pre><code>` with no wrapping `role="region" tabindex="0"` container, so a horizontally overflowing block couldn't be scrolled into view by a keyboard-only user the way the equivalent code blocks on this author's other project, pgdmn, can. Fixed in `src/markdown.rs` by wrapping every fenced code block's `<pre><code>` in a `<div role="region" tabindex="0" aria-label="Code sample">`.

## SEO

### SEO-001: Twitter card and locale metadata

Pages carry Open Graph tags but no `twitter:card` / `twitter:title` / `twitter:image` tags and no `og:locale`. Most platforms fall back to Open Graph so the impact is small, but X/Twitter renders richer previews with explicit card tags. When adding them, consolidate the per-page head metadata (canonical, description, Open Graph) into one shared component so each page declares its facts once — today the block is hand-repeated in each page template, and a per-page omission is invisible.

## Images

### IMG-001: Explicit dimensions and lazy loading for article images

Images rendered from Markdown carry no `width`/`height` attributes (so the layout shifts as they load) and no `loading="lazy"`/`decoding="async"` hints. Fixing this means reading each image's pixel dimensions at build time and post-processing the rendered `img` tags in `src/markdown.rs`, the way code blocks are already post-processed.

### IMG-002: Modern formats and responsive variants for article images

`public/` holds unoptimized original PNG/JPEG files served at full size to every device. Generate compressed WebP/AVIF versions and responsive `srcset` variants at build time, keeping the originals as the fallback.

## Blog Authoring

### BLOG-001: Local preview port is hardcoded in two places

`Makefile`'s `PORT` variable (used to build the URL `make blog` opens) and the `127.0.0.1:4000` bind address in `src/bin/serve.rs` are two separate hardcoded copies of the same port number. Changing one without the other would make `make blog` open the wrong URL silently. Consider threading the port through a single source of truth (e.g. an environment variable `serve` reads, with the Makefile setting it) if the port ever needs to change.
