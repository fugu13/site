pub const POST: &str = "post";
pub const NOT_FOUND: &str = "404";
pub const DOMAIN: &str = "https://www.russellduhon.com";

pub fn post(slug: &str) -> String {
    format!("/{POST}/{slug}/")
}

/// Absolute production URL for a site-root-relative path (must start with `/`).
pub fn absolute(path: &str) -> String {
    format!("{DOMAIN}{path}")
}

/// Absolute production URL of a post page — the one canonical shape the
/// sitemap, feed, structured data, and canonical link all share.
pub fn post_url(slug: &str) -> String {
    absolute(&post(slug))
}
