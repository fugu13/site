# Search Indexing & Syndication

The site publishes everything a search engine or feed reader needs to discover, index, and present its pages accurately — a sitemap, an Atom feed, robots directives, canonical URLs, per-page titles and descriptions, and per-post structured data — all generated at build time from the same parsed posts the pages render from.

## What this lets a user do

- Find the site and its posts through a search engine, with each post indexed at exactly one canonical address.
- See an informative search result for a post: a title naming both the post and the author, and a snippet drawn from the post's own description rather than scraped body text.
- Subscribe to the blog in a feed reader and be notified of new posts without revisiting the site.
- Share a post anywhere and have the link resolve to the same URL a search engine indexes, so shares and search reinforce each other.

## The pieces

| File or tag | What it does |
|---|---|
| `sitemap.xml` | Lists the home page and every published post with its publication date — or its front-matter `updated` date, when a post has been revised — as `lastmod`. No `changefreq` or `priority` (search engines ignore them), and no synthetic dates — search engines stop trusting `lastmod` on a site once it proves inaccurate, so it only ever carries real front-matter dates. Drafts are excluded automatically because they are excluded from the parsed post list itself. |
| `atom.xml` | Atom feed of every published post, newest first, each entry carrying the post's title, canonical link, publication date, and front-matter description as its summary. Every page's head advertises it via a `rel="alternate"` link so feed readers can autodiscover it from any URL. |
| `robots.txt` | Allows all crawling and names the sitemap's absolute URL, so crawlers find the sitemap with no manual submission. |
| `llms.txt` | A Markdown guide for AI crawlers and assistants, per the llmstxt.org convention: the site's name, its one-sentence description, and a link to every published post with its description. |
| Canonical link | Every indexable page declares its one true URL — on the `www` host, with a trailing slash — matching the shape the static host actually serves without redirects. |
| Titles | The browser-tab and search-result title names the page first and the author after a middot, per the formats specified in `docs/ux/page-titles.md`. |
| Meta description | Each post's front-matter `description` doubles as its search-snippet description; the home page uses the site's one-sentence description, which the `WebSite` structured data and `llms.txt` share. A post with no description simply has no description tag — nothing is synthesized. |
| Open Graph tags | Each page declares its title, type, canonical URL, description, image, and site name for link previews on social and messaging apps. |
| Favicon | An "R" monogram in the site's colors, served as an SVG with a PNG fallback and linked from every page's head — the icon browsers show in tabs and search engines show beside results. |
| Structured data | The home page embeds a JSON-LD graph of a `WebSite` node and a `Person` node for the author (name, site URL, LinkedIn identity); each post page embeds a `BlogPosting` node — headline, publication and modification dates, canonical URL, description and image when present — whose author is that same `Person`, carried by a stable identifier so every page names one consistent identity. This is the one inline script form the site's no-JavaScript convention permits: it is inert data, never executed. |

## Where it comes from

Everything above derives from the front matter and slugs of the posts themselves, plus a handful of site identity constants — the production origin, the site's name and one-sentence description, and the author's name and LinkedIn profile. Adding a post updates the sitemap, feed, `llms.txt`, and home page in the same build with no further steps; there is no separately maintained URL list to fall out of date.

## What is deliberately not here

- No `lastmod` freshness games, no priority hints: only verifiable facts go in the sitemap.
- No Twitter-specific card tags or locale tags yet — tracked as SEO-001 in `TODO.md`.
- Search-console registration and the apex-domain DNS record are operational concerns outside the repository.
