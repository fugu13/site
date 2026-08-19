//! Build entry point: renders every route to static HTML in `dist/`.
//!
//! Run via `make build` (`cargo run --release --bin prerender`). There is no server in
//! production — this binary is the entire build step, and `dist/` is the deployable
//! artifact.

use std::error::Error;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::Path;

use leptos::prelude::LeptosOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let dist = Path::new("dist");

    reset_dir(dist)?;
    let stylesheet_path = compile_stylesheet(Path::new("style/main.scss"), dist)?;
    println!("Stylesheet written to dist{stylesheet_path}");
    site::assets::set_stylesheet_path(stylesheet_path);
    copy_dir_contents(Path::new("public"), dist)?;

    // Calling `all()` (rather than `load()`) both parses every post now — so a
    // broken post fails the build immediately, by filename — and populates the
    // cache every render during route generation below reads from, instead of
    // silently reparsing (and re-highlighting) all of them a second time.
    println!("Loaded {} post(s).", site::posts::all().len());

    let options = LeptosOptions::builder()
        .output_name("site")
        .site_root("dist")
        .build();

    let (_routes, static_routes) = leptos_axum::generate_route_list_with_ssg(site::app::shell);
    static_routes.generate(&options).await;

    nest_html_files(dist)?;
    strip_inert_markup_in_dir(dist)?;

    for (path, content) in site::seo::machine_outputs(site::posts::all()) {
        // The paths are URL-space constants; the leading slash comes off to
        // land each file at dist/'s top level.
        std::fs::write(dist.join(path.trim_start_matches('/')), content)?;
    }

    std::fs::write(dist.join(".nojekyll"), "")?;

    Ok(())
}

/// Remove `dir` if present and recreate it empty.
fn reset_dir(dir: &Path) -> std::io::Result<()> {
    if dir.exists() {
        std::fs::remove_dir_all(dir)?;
    }
    std::fs::create_dir_all(dir)
}

/// Compile the site's SCSS entry point to a single compressed CSS file,
/// named with a hash of its own content (`style.<hash>.css`) so a change to
/// the stylesheet gets a new URL instead of relying on a visitor's browser
/// to notice a cache-control header. Returns the site-root-absolute path to
/// link (e.g. `/style.a1b2c3d4.css`).
fn compile_stylesheet(source: &Path, dist: &Path) -> Result<String, Box<dyn Error>> {
    let css = grass::from_path(
        source,
        &grass::Options::default().style(grass::OutputStyle::Compressed),
    )?;
    let filename = format!("style.{}.css", content_hash(css.as_bytes()));
    std::fs::write(dist.join(&filename), css)?;
    Ok(format!("/{filename}"))
}

/// A short, deterministic-within-this-build fingerprint of `bytes`, used only
/// for cache busting — collision resistance and stability across Rust
/// versions don't matter here, only that the same content hashes the same
/// way within one `prerender` run and changed content hashes differently.
#[allow(
    clippy::cast_possible_truncation,
    reason = "deliberately keeping only the low 32 bits of the hash for a short filename fragment"
)]
fn content_hash(bytes: &[u8]) -> String {
    let mut hasher = DefaultHasher::new();
    bytes.hash(&mut hasher);
    format!("{:08x}", hasher.finish() as u32)
}

/// Recursively copy every file under `src` into `dest`, preserving relative paths but
/// not nesting under `src`'s own directory name (e.g. `public/foo.png` becomes
/// `dist/foo.png`, not `dist/public/foo.png`).
fn copy_dir_contents(src: &Path, dest: &Path) -> std::io::Result<()> {
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let target = dest.join(entry.file_name());
        if file_type.is_dir() {
            std::fs::create_dir_all(&target)?;
            copy_dir_contents(&entry.path(), &target)?;
        } else {
            std::fs::copy(entry.path(), &target)?;
        }
    }
    Ok(())
}

/// Recursively rewrite every `foo.html` file under `dir` to `foo/index.html`, so static
/// hosts serve it at the extensionless `/foo/` URL Leptos's router produces. Two kinds
/// of file are left alone: any file already named `index.html`, and — only at `dir`'s
/// own top level — `404.html`, which static hosts look for by that exact name.
fn nest_html_files(dir: &Path) -> std::io::Result<()> {
    nest_html_files_at(dir, dir)
}

fn nest_html_files_at(dir: &Path, root: &Path) -> std::io::Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if entry.file_type()?.is_dir() {
            nest_html_files_at(&path, root)?;
            continue;
        }

        if path.extension().and_then(|ext| ext.to_str()) != Some("html") {
            continue;
        }
        let stem = path.file_stem().and_then(|stem| stem.to_str());
        if stem == Some("index") {
            continue;
        }
        if dir == root && stem == Some(site::routes::NOT_FOUND) {
            continue;
        }

        let Some(stem) = stem else { continue };
        let nested_dir = dir.join(stem);
        std::fs::create_dir_all(&nested_dir)?;
        std::fs::rename(&path, nested_dir.join("index.html"))?;
    }
    Ok(())
}

/// Recursively strip inert Leptos markup — hydration `<script>` elements and empty
/// fragment markers — from every `.html` file under `dir`, leaving every real
/// (`src`-bearing) script — the site's three analytics scripts — and every JSON-LD
/// data block untouched wherever they appear. The CI workflow independently verifies
/// that no script's `src` falls outside the three approved analytics domains, that no
/// other inline script survives, and that no fragment marker remains.
fn strip_inert_markup_in_dir(dir: &Path) -> std::io::Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if entry.file_type()?.is_dir() {
            strip_inert_markup_in_dir(&path)?;
            continue;
        }
        if path.extension().and_then(|ext| ext.to_str()) != Some("html") {
            continue;
        }
        let html = std::fs::read_to_string(&path)?;
        let stripped = strip_inert_markup(&html);
        if stripped != html {
            std::fs::write(&path, stripped)?;
        }
    }
    Ok(())
}

/// Remove Leptos's two kinds of inert output markup in one pass.
///
/// First, every `<script>...</script>` element whose opening tag has no `src`
/// attribute, except `type="application/ld+json"` data blocks — inert structured data
/// for search engines, deliberately kept even though they are inline (the JSON they
/// carry never executes). Leptos's own inert hydration bookkeeping is always emitted
/// inline (no `src`); every real script this site ever loads — its three analytics
/// scripts — always has one. This deletes the former while preserving the latter
/// verbatim, in whatever order and position they appear, without having to name
/// specific hosts here.
///
/// Second, every `<!>` empty fragment marker — a placeholder Leptos emits between
/// dynamic view children, meaningless in output that never hydrates and malformed
/// markup to strict parsers. Sweeping the final bytes rather than fixing each view
/// is deliberate: it also catches markers from views nobody has written yet.
fn strip_inert_markup(html: &str) -> String {
    const MARKER: &str = "<!>";
    let mut result = String::with_capacity(html.len());
    let mut rest = html;

    loop {
        let next_script = rest.find("<script");
        let next_marker = rest.find(MARKER);
        let (start, is_marker) = match (next_script, next_marker) {
            (None, None) => break,
            (Some(script), None) => (script, false),
            (None, Some(marker)) => (marker, true),
            (Some(script), Some(marker)) => (script.min(marker), marker < script),
        };
        result.push_str(&rest[..start]);

        if is_marker {
            rest = &rest[start + MARKER.len()..];
            continue;
        }

        let tail = &rest[start..];
        let Some(tag_end) = tail.find('>').map(|i| i + 1) else {
            // No closing `>` on the opening tag at all: malformed, keep verbatim and stop.
            result.push_str(tail);
            rest = "";
            break;
        };
        let opening_tag = &tail[..tag_end];
        let after_open = &tail[tag_end..];

        let (element, remainder) = match after_open.find("</script>") {
            Some(close) => {
                let end = close + "</script>".len();
                (&tail[..tag_end + end], &after_open[end..])
            }
            // No closing tag found: keep the rest of the document as part of this
            // (malformed) element rather than risk truncating real content.
            None => (tail, ""),
        };

        if opening_tag.contains(" src=") || opening_tag.contains("application/ld+json") {
            result.push_str(element);
        }
        rest = remainder;
    }
    result.push_str(rest);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn content_hash_is_deterministic_for_the_same_bytes() {
        assert_eq!(
            content_hash(b"p { color: red; }"),
            content_hash(b"p { color: red; }")
        );
    }

    #[test]
    fn content_hash_changes_when_content_changes() {
        assert_ne!(
            content_hash(b"p { color: red; }"),
            content_hash(b"p { color: blue; }")
        );
    }

    #[test]
    fn allowed_analytics_script_survives_unchanged() {
        let html = r#"<head><script defer="" src="https://app.tinyanalytics.io/pixel/MB6jAtnTO5M0SZ9n"></script></head>"#;
        assert_eq!(strip_inert_markup(html), html);
    }

    #[test]
    fn unrecognized_script_is_removed() {
        let html = r#"<body><script>(function(){window.__leptos_hydrate=1;})();</script><p>"hi"</p></body>"#;
        let expected = r#"<body><p>"hi"</p></body>"#;
        assert_eq!(strip_inert_markup(html), expected);
    }

    #[test]
    fn empty_fragment_markers_are_removed() {
        let html = "<div><!><p>content</p><!></div>";
        assert_eq!(strip_inert_markup(html), "<div><p>content</p></div>");
    }

    #[test]
    fn fragment_marker_before_a_kept_script_leaves_the_script_alone() {
        let html = r#"<!><script defer="" src="https://plausible.io/js/script.js"></script>"#;
        let expected = r#"<script defer="" src="https://plausible.io/js/script.js"></script>"#;
        assert_eq!(strip_inert_markup(html), expected);
    }

    #[test]
    fn json_ld_data_block_survives_unchanged() {
        let html = r#"<main><script type="application/ld+json">{"@type":"BlogPosting","headline":"Hi"}</script></main>"#;
        assert_eq!(strip_inert_markup(html), html);
    }

    #[test]
    fn script_free_html_is_untouched() {
        let html = "<main><h1>Russell Duhon</h1><p>No scripts here.</p></main>";
        assert_eq!(strip_inert_markup(html), html);
    }

    #[test]
    fn mixed_scripts_keep_only_those_with_src() {
        let html = concat!(
            r#"<script>hydrate();</script>"#,
            r#"<script defer="" async="" src="https://scripts.simpleanalyticscdn.com/latest.js"></script>"#,
            r#"<script defer="" src="https://plausible.io/js/script.js" data-domain="russellduhon.com"></script>"#,
        );
        let expected = concat!(
            r#"<script defer="" async="" src="https://scripts.simpleanalyticscdn.com/latest.js"></script>"#,
            r#"<script defer="" src="https://plausible.io/js/script.js" data-domain="russellduhon.com"></script>"#,
        );
        assert_eq!(strip_inert_markup(html), expected);
    }

    #[test]
    fn a_src_bearing_script_from_an_unrecognized_host_still_survives_this_pass() {
        // Deliberate: this function's job is separating real scripts from Leptos's
        // inert inline bookkeeping, not vetting domains — that's the CI workflow's
        // independent, stricter check (only the three named analytics domains).
        let html = r#"<script src="https://example.test/whatever.js"></script>"#;
        assert_eq!(strip_inert_markup(html), html);
    }
}
