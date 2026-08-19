use std::sync::OnceLock;

use chrono::{DateTime, FixedOffset};
use include_dir::{Dir, File, include_dir};
use serde::Deserialize;

static ARTICLES: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/articles");

/// A single blog post: parsed front matter plus the Markdown body rendered to HTML.
pub struct Post {
    pub slug: String,
    pub title: String,
    pub date: DateTime<FixedOffset>,
    /// When the post was last substantively edited, if ever — the optional
    /// `updated` front matter field.
    pub updated: Option<DateTime<FixedOffset>>,
    pub description: Option<String>,
    pub image: Option<String>,
    pub html: String,
}

#[derive(Deserialize)]
struct FrontMatter {
    title: String,
    date: DateTime<FixedOffset>,
    #[serde(default)]
    updated: Option<DateTime<FixedOffset>>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    image: Option<String>,
    #[serde(default)]
    draft: bool,
}

static POSTS: OnceLock<Vec<Post>> = OnceLock::new();

/// Every post, newest first.
///
/// Parsed once and cached; a post that fails to parse is a build-time bug,
/// so this panics rather than threading a `Result` through every call site —
/// use [`load`] directly if you need to handle the error yourself.
pub fn all() -> &'static [Post] {
    POSTS
        .get_or_init(|| load().unwrap_or_else(|err| panic!("{err}")))
        .as_slice()
}

/// Look up a single post by its slug (the filename stem of its source file).
pub fn by_slug(slug: &str) -> Option<&'static Post> {
    all().iter().find(|post| post.slug == slug)
}

/// The slug of every post, in the same newest-first order as [`all`].
pub fn slugs() -> Vec<String> {
    all().iter().map(|post| post.slug.clone()).collect()
}

/// Parse every non-draft article embedded from `articles/` into a [`Post`], newest first.
///
/// An article whose front matter sets `draft: true` is skipped entirely — it gets no
/// route and never appears in [`all`], [`by_slug`], or [`slugs`].
pub fn load() -> Result<Vec<Post>, String> {
    let mut posts = Vec::new();
    for file in ARTICLES.files() {
        if let Some(post) = parse_post(file)? {
            posts.push(post);
        }
    }
    posts.sort_by_key(|post| std::cmp::Reverse(post.date));
    Ok(posts)
}

fn parse_post(file: &File<'_>) -> Result<Option<Post>, String> {
    let path = file.path();
    let name = path.to_string_lossy();
    let slug = path
        .file_stem()
        .map(|stem| stem.to_string_lossy().into_owned())
        .ok_or_else(|| format!("{name}: could not determine slug from filename"))?;
    let source = file
        .contents_utf8()
        .ok_or_else(|| format!("{name}: file is not valid UTF-8"))?;
    parse_post_source(&name, slug, source)
}

fn parse_post_source(name: &str, slug: String, source: &str) -> Result<Option<Post>, String> {
    let (front_matter, body) = split_front_matter(source)
        .ok_or_else(|| format!("{name}: missing or malformed YAML front matter"))?;
    let front_matter: FrontMatter = serde_yaml::from_str(front_matter)
        .map_err(|err| format!("{name}: failed to parse front matter: {err}"))?;
    if front_matter.draft {
        return Ok(None);
    }
    let html = crate::markdown::to_html(body);
    let image = front_matter.image.or_else(|| first_src(&html));

    Ok(Some(Post {
        slug,
        title: front_matter.title,
        date: front_matter.date,
        updated: front_matter.updated,
        description: front_matter.description,
        image,
        html,
    }))
}

/// Split `---`-delimited YAML front matter from the remaining Markdown body.
fn split_front_matter(source: &str) -> Option<(&str, &str)> {
    let rest = source.strip_prefix("---")?;
    let rest = rest
        .strip_prefix("\r\n")
        .or_else(|| rest.strip_prefix('\n'))?;
    let end = rest.find("\n---")?;
    let front_matter = &rest[..end];
    let after = &rest[end + "\n---".len()..];
    let body = after
        .strip_prefix("\r\n")
        .or_else(|| after.strip_prefix('\n'))
        .unwrap_or(after);
    Some((front_matter, body))
}

/// The value of the first `src="..."` attribute found anywhere in `html`. A plain
/// substring search, matching the legacy regex-based fallback this replaces (it also
/// matched any `src` attribute, not specifically `<img>` tags).
fn first_src(html: &str) -> Option<String> {
    let start = html.find("src=\"")? + "src=\"".len();
    let end = html[start..].find('"')?;
    Some(html[start..start + end].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn folded_block_description_parses_as_single_string() {
        let posts = load().expect("posts should load");
        let post = posts
            .iter()
            .find(|post| post.slug == "beautiful-postgresql-templates")
            .expect("beautiful-postgresql-templates.md should be present");
        let description = post
            .description
            .as_deref()
            .expect("post should have a description");
        assert!(!description.trim().is_empty());
    }

    #[test]
    fn missing_front_matter_image_falls_back_to_rendered_html() {
        let posts = load().expect("posts should load");
        let post = posts
            .iter()
            .find(|post| post.slug == "messily-thinking-pythonically")
            .expect("messily-thinking-pythonically.md should be present");
        let image = post.image.as_deref().expect("post should have an image");
        assert!(!image.is_empty());
    }

    #[test]
    fn draft_front_matter_excludes_the_post() {
        let source = "---\ntitle: \"Draft\"\ndate: \"2026-01-01T00:00:00-08:00\"\ndraft: true\n---\n\nBody.\n";
        let post = parse_post_source("draft.md", "draft".to_string(), source)
            .expect("front matter should parse");
        assert!(post.is_none());
    }

    #[test]
    fn missing_draft_field_defaults_to_published() {
        let source =
            "---\ntitle: \"Published\"\ndate: \"2026-01-01T00:00:00-08:00\"\n---\n\nBody.\n";
        let post = parse_post_source("post.md", "post".to_string(), source)
            .expect("front matter should parse");
        assert!(post.is_some());
    }

    #[test]
    fn updated_front_matter_is_optional_and_parses_when_present() {
        let without = "---\ntitle: \"P\"\ndate: \"2026-01-01T00:00:00-08:00\"\n---\n\nBody.\n";
        let post = parse_post_source("p.md", "p".to_string(), without)
            .expect("front matter should parse")
            .expect("post should be published");
        assert!(post.updated.is_none());

        let with = "---\ntitle: \"P\"\ndate: \"2026-01-01T00:00:00-08:00\"\nupdated: \"2026-02-01T00:00:00-08:00\"\n---\n\nBody.\n";
        let post = parse_post_source("p.md", "p".to_string(), with)
            .expect("front matter should parse")
            .expect("post should be published");
        assert!(post.updated.is_some());
    }
}
