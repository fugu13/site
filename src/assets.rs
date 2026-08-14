//! The stylesheet's cache-busting path, threaded from `prerender` into every
//! rendered page.
//!
//! `prerender` computes this once, from the compiled CSS's own content hash,
//! before rendering any route — every page then links the exact same
//! (correct) path.

use std::sync::OnceLock;

static STYLESHEET_PATH: OnceLock<String> = OnceLock::new();

/// Set by `prerender`, exactly once, before any page renders.
pub fn set_stylesheet_path(path: String) {
    STYLESHEET_PATH
        .set(path)
        .expect("set_stylesheet_path must be called exactly once, before any page renders");
}

/// The path pages should link to. Falls back to the unhashed `/style.css`
/// if never set, so a page rendered outside of `prerender` (e.g. directly in
/// a unit test) still has a sane value rather than panicking.
pub fn stylesheet_path() -> &'static str {
    STYLESHEET_PATH
        .get()
        .map(String::as_str)
        .unwrap_or("/style.css")
}
