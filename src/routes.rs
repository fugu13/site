pub const POST: &str = "post";
pub const NOT_FOUND: &str = "404";
pub const DOMAIN: &str = "https://www.russellduhon.com";

pub fn post(slug: &str) -> String {
    format!("/{POST}/{slug}/")
}
