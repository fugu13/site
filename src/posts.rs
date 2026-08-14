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
    pub description: Option<String>,
    pub image: Option<String>,
    pub html: String,
}

#[derive(Deserialize)]
struct FrontMatter {
    title: String,
    date: DateTime<FixedOffset>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    image: Option<String>,
}

static POSTS: OnceLock<Vec<Post>> = OnceLock::new();

/// Every post, newest first. Parsed once and cached; a post that fails to parse is a
/// build-time bug, so this panics rather than threading a `Result` through every call
/// site — use [`load`] directly if you need to handle the error yourself.
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

/// Parse every article embedded from `articles/` into a [`Post`], newest first.
pub fn load() -> Result<Vec<Post>, String> {
    let mut posts = ARTICLES
        .files()
        .map(parse_post)
        .collect::<Result<Vec<_>, _>>()?;
    posts.sort_by_key(|post| std::cmp::Reverse(post.date));
    Ok(posts)
}

fn parse_post(file: &File<'_>) -> Result<Post, String> {
    let path = file.path();
    let name = path.to_string_lossy();
    let slug = path
        .file_stem()
        .map(|stem| stem.to_string_lossy().into_owned())
        .ok_or_else(|| format!("{name}: could not determine slug from filename"))?;
    let source = file
        .contents_utf8()
        .ok_or_else(|| format!("{name}: file is not valid UTF-8"))?;
    let (front_matter, body) = split_front_matter(source)
        .ok_or_else(|| format!("{name}: missing or malformed YAML front matter"))?;
    let front_matter: FrontMatter = serde_yaml::from_str(front_matter)
        .map_err(|err| format!("{name}: failed to parse front matter: {err}"))?;
    let html = crate::markdown::to_html(body);
    let image = front_matter.image.or_else(|| first_src(&html));

    Ok(Post {
        slug,
        title: front_matter.title,
        date: front_matter.date,
        description: front_matter.description,
        image,
        html,
    })
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
}
