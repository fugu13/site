//! The site's machine-readable SEO surface and shared identity constants.
//!
//! Covers the XML sitemap, Atom feed, `robots.txt`, `llms.txt`, and
//! schema.org JSON-LD structured data, plus the site identity constants those
//! outputs and the page templates share. Everything generated here derives
//! from the same parsed posts the pages render from, so none of it can drift
//! from the routes that actually exist.

use std::borrow::Cow;
use std::fmt::Write;

use crate::posts::Post;
use crate::routes;

/// The blog's name as presented to feed readers, social previews, and
/// structured data.
pub const SITE_TITLE: &str = "Russell Duhon's Blog";

/// The site author's name, used in page titles, the feed, and structured data.
pub const AUTHOR: &str = "Russell Duhon";

/// One-sentence description of the site, used as the home page's meta
/// description, the `WebSite` structured data description, and the
/// `llms.txt` summary.
pub const SITE_DESCRIPTION: &str = "Russell Duhon writes about software development topics such as DMN, Rust, property-based testing, PostgreSQL, WebAssembly, and Python.";

/// The author's `LinkedIn` profile, linked from the home page bio and named
/// as the `Person` structured data's `sameAs` identity.
pub const LINKEDIN_URL: &str = "https://www.linkedin.com/in/russell-duhon-322a0244";

/// Site-root-relative path of the generated sitemap.
pub const SITEMAP_PATH: &str = "/sitemap.xml";

/// Site-root-relative path of the generated Atom feed.
pub const FEED_PATH: &str = "/atom.xml";

/// Site-root-relative path of the generated robots file.
pub const ROBOTS_PATH: &str = "/robots.txt";

/// Site-root-relative path of the generated llms.txt.
pub const LLMS_PATH: &str = "/llms.txt";

/// The browser-tab / search-result title of a page: the page's own name
/// first, the author after a middot — the one convention
/// `docs/ux/page-titles.md` documents.
pub fn page_title(page: &str) -> String {
    format!("{page} · {AUTHOR}")
}

/// Every machine-readable file the build writes, as (URL-space path, content)
/// pairs — the one list `prerender` walks.
///
/// CI re-checks the same filenames independently and by name, deliberately,
/// so a new output is added here and in the workflow's required-files list.
pub fn machine_outputs(posts: &[Post]) -> Vec<(&'static str, String)> {
    vec![
        (SITEMAP_PATH, sitemap_xml(posts)),
        (FEED_PATH, atom_feed(posts)),
        (ROBOTS_PATH, robots_txt()),
        (LLMS_PATH, llms_txt(posts)),
    ]
}

/// The XML sitemap: the home page plus every published post.
///
/// `lastmod` carries only real front-matter dates — the home page uses the
/// newest post's date, since publishing a post is what changes it. There is
/// deliberately no `changefreq` or `priority` (search engines ignore them),
/// and no synthetic "today" dates (search engines stop trusting `lastmod`
/// entirely once it proves inaccurate).
pub fn sitemap_xml(posts: &[Post]) -> String {
    let mut xml = String::from(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
"#,
    );
    if let Some(newest) = posts.first() {
        push_sitemap_url(&mut xml, &routes::home_url(), &newest.date.to_rfc3339());
    }
    for post in posts {
        push_sitemap_url(
            &mut xml,
            &routes::post_url(&post.slug),
            &post.date.to_rfc3339(),
        );
    }
    xml.push_str("</urlset>\n");
    xml
}

fn push_sitemap_url(xml: &mut String, loc: &str, lastmod: &str) {
    // Writing to a String is infallible, so the Result is safely ignored.
    let _ = writeln!(
        xml,
        "<url><loc>{}</loc><lastmod>{lastmod}</lastmod></url>",
        xml_escape(loc)
    );
}

/// The Atom feed: one entry per published post, newest first, with the
/// front-matter description as each entry's summary.
pub fn atom_feed(posts: &[Post]) -> String {
    // Atom requires a feed-level <updated>; the empty fallback is unreachable
    // while the site has any published post at all.
    let updated = posts
        .first()
        .map_or_else(String::new, |post| post.date.to_rfc3339());
    let mut xml = format!(
        r#"<?xml version="1.0" encoding="utf-8"?>
<feed xmlns="http://www.w3.org/2005/Atom">
<title>{title}</title>
<id>{home}</id>
<link href="{home}"/>
<link rel="self" href="{feed_url}"/>
<updated>{updated}</updated>
<author><name>{author}</name></author>
"#,
        title = xml_escape(SITE_TITLE),
        home = routes::home_url(),
        feed_url = routes::absolute(FEED_PATH),
        author = xml_escape(AUTHOR),
    );
    for post in posts {
        let post_url = routes::post_url(&post.slug);
        let url = xml_escape(&post_url);
        // Writing to a String is infallible, so the Results are safely ignored.
        xml.push_str("<entry>\n");
        let _ = writeln!(xml, "<title>{}</title>", xml_escape(&post.title));
        let _ = writeln!(xml, "<id>{url}</id>");
        let _ = writeln!(xml, "<link href=\"{url}\"/>");
        let _ = writeln!(xml, "<updated>{}</updated>", post.date.to_rfc3339());
        if let Some(description) = &post.description {
            let _ = writeln!(xml, "<summary>{}</summary>", xml_escape(description));
        }
        xml.push_str("</entry>\n");
    }
    xml.push_str("</feed>\n");
    xml
}

/// `robots.txt`: allow everything and point crawlers at the sitemap.
pub fn robots_txt() -> String {
    format!(
        "User-agent: *\nAllow: /\n\nSitemap: {}\n",
        routes::absolute(SITEMAP_PATH)
    )
}

/// `llms.txt`: a Markdown site guide for AI crawlers and assistants, per the
/// llmstxt.org convention — the site's name, its one-sentence description,
/// and a link to every published post.
pub fn llms_txt(posts: &[Post]) -> String {
    let mut text = format!("# {SITE_TITLE}\n\n> {SITE_DESCRIPTION}\n\n## Posts\n\n");
    for post in posts {
        let url = routes::post_url(&post.slug);
        let description = post
            .description
            .as_deref()
            .map(|description| format!(": {description}"))
            .unwrap_or_default();
        // Writing to a String is infallible, so the Result is safely ignored.
        let _ = writeln!(text, "- [{}]({url}){description}", post.title);
    }
    text
}

/// The stable JSON-LD `@id` that names the author's `Person` node, used by
/// the node itself and by every reference to it — one definition, so the
/// graph link can never dangle.
fn person_id() -> String {
    format!("{}#person", routes::home_url())
}

/// The author's schema.org `Person` node. The home page's graph defines it
/// once and links it by `@id`; each post page inlines a full copy so the post
/// document stands alone for a crawler that fetches only it.
fn person_json_ld() -> serde_json::Value {
    serde_json::json!({
        "@type": "Person",
        "@id": person_id(),
        "name": AUTHOR,
        "url": routes::home_url(),
        "sameAs": [LINKEDIN_URL],
    })
}

/// The home page's JSON-LD: a `WebSite` node plus the author's `Person` node,
/// serialized ready to embed in a `<script type="application/ld+json">` data
/// block.
pub fn home_json_ld() -> String {
    let home = routes::home_url();
    serialize_for_script(&serde_json::json!({
        "@context": "https://schema.org",
        "@graph": [
            {
                "@type": "WebSite",
                "@id": format!("{home}#website"),
                "url": home,
                "name": SITE_TITLE,
                "description": SITE_DESCRIPTION,
                "author": { "@id": person_id() },
            },
            person_json_ld(),
        ],
    }))
}

/// One post's `BlogPosting` JSON-LD, serialized ready to embed in a
/// `<script type="application/ld+json">` data block.
pub fn blog_posting_json_ld(post: &Post) -> String {
    let url = routes::post_url(&post.slug);
    let mut value = serde_json::json!({
        "@context": "https://schema.org",
        "@type": "BlogPosting",
        "headline": post.title,
        "datePublished": post.date.to_rfc3339(),
        "url": url,
        "mainEntityOfPage": url,
        "author": person_json_ld(),
    });
    if let Some(description) = &post.description {
        value["description"] = serde_json::Value::String(description.clone());
    }
    if let Some(image) = &post.image {
        value["image"] = serde_json::Value::String(routes::absolute(image));
    }
    serialize_for_script(&value)
}

/// Serialize a JSON-LD value for embedding in a script element. Every angle
/// bracket is replaced with the JSON string escape "backslash u003c" (still
/// plain JSON) so no string value can ever contain a literal closing script
/// tag and end the surrounding element early.
fn serialize_for_script(value: &serde_json::Value) -> String {
    value.to_string().replace('<', "\\u003c")
}

/// Escape XML metacharacters, borrowing the input unchanged when it has none.
fn xml_escape(text: &str) -> Cow<'_, str> {
    if !text.contains(['&', '<', '>', '"', '\'']) {
        return Cow::Borrowed(text);
    }
    let mut escaped = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&apos;"),
            _ => escaped.push(c),
        }
    }
    Cow::Owned(escaped)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::DateTime;

    fn post(slug: &str, title: &str, description: Option<&str>, image: Option<&str>) -> Post {
        Post {
            slug: slug.to_owned(),
            title: title.to_owned(),
            date: DateTime::parse_from_rfc3339("2026-01-02T03:04:05-08:00")
                .expect("test date should parse"),
            description: description.map(str::to_owned),
            image: image.map(str::to_owned),
            html: String::new(),
        }
    }

    #[test]
    fn page_title_puts_the_page_before_the_author() {
        assert_eq!(page_title("Some Post"), "Some Post · Russell Duhon");
    }

    #[test]
    fn machine_outputs_cover_sitemap_feed_robots_and_llms() {
        let outputs = machine_outputs(&[post("only", "Only", None, None)]);
        let paths: Vec<_> = outputs.iter().map(|(path, _)| *path).collect();
        assert_eq!(paths, vec![SITEMAP_PATH, FEED_PATH, ROBOTS_PATH, LLMS_PATH]);
        assert!(outputs.iter().all(|(_, content)| !content.is_empty()));
    }

    #[test]
    fn llms_txt_lists_every_post_with_its_description() {
        let posts = [
            post("plain", "Plain", None, None),
            post("described", "Described", Some("What it covers."), None),
        ];
        let text = llms_txt(&posts);
        assert!(text.starts_with("# Russell Duhon's Blog\n\n> "));
        assert!(text.contains("- [Plain](https://www.russellduhon.com/post/plain/)\n"));
        assert!(text.contains(
            "- [Described](https://www.russellduhon.com/post/described/): What it covers.\n"
        ));
    }

    #[test]
    fn home_json_ld_carries_website_and_person_nodes() {
        let value: serde_json::Value =
            serde_json::from_str(&home_json_ld()).expect("home JSON-LD should be valid JSON");
        let graph = value["@graph"].as_array().expect("should have a @graph");
        assert_eq!(graph[0]["@type"], "WebSite");
        assert_eq!(graph[1]["@type"], "Person");
        assert_eq!(graph[1]["@id"], "https://www.russellduhon.com/#person");
        // The graph only links if the reference and the node carry the same id.
        assert_eq!(graph[0]["author"]["@id"], graph[1]["@id"]);
        assert_eq!(graph[1]["sameAs"][0], LINKEDIN_URL);
    }

    #[test]
    fn sitemap_lists_home_and_every_post_with_lastmod() {
        let posts = [
            post("first", "First", None, None),
            post("second", "Second", None, None),
        ];
        let xml = sitemap_xml(&posts);
        assert_eq!(xml.matches("<url>").count(), 3);
        assert!(xml.contains("<loc>https://www.russellduhon.com/</loc>"));
        assert!(xml.contains("<loc>https://www.russellduhon.com/post/first/</loc>"));
        assert!(xml.contains("<loc>https://www.russellduhon.com/post/second/</loc>"));
        assert_eq!(
            xml.matches("<lastmod>2026-01-02T03:04:05-08:00</lastmod>")
                .count(),
            3
        );
    }

    #[test]
    fn sitemap_omits_changefreq_and_priority() {
        let xml = sitemap_xml(&[post("only", "Only", None, None)]);
        assert!(!xml.contains("changefreq"));
        assert!(!xml.contains("priority"));
    }

    #[test]
    fn atom_feed_escapes_titles_and_carries_summaries() {
        let posts = [post("amps", "Bits & <Pieces>", Some("A summary."), None)];
        let xml = atom_feed(&posts);
        assert!(xml.contains("<title>Bits &amp; &lt;Pieces&gt;</title>"));
        assert!(xml.contains("<summary>A summary.</summary>"));
        assert!(xml.contains("<link href=\"https://www.russellduhon.com/post/amps/\"/>"));
        assert!(xml.contains("<updated>2026-01-02T03:04:05-08:00</updated>"));
    }

    #[test]
    fn robots_points_at_the_sitemap() {
        let robots = robots_txt();
        assert!(robots.contains("Sitemap: https://www.russellduhon.com/sitemap.xml"));
    }

    #[test]
    fn json_ld_round_trips_and_carries_the_optional_fields() {
        let serialized = blog_posting_json_ld(&post(
            "full",
            "Full Post",
            Some("The description."),
            Some("/image.png"),
        ));
        let value: serde_json::Value =
            serde_json::from_str(&serialized).expect("JSON-LD should be valid JSON");
        assert_eq!(value["@type"], "BlogPosting");
        assert_eq!(value["headline"], "Full Post");
        assert_eq!(value["url"], "https://www.russellduhon.com/post/full/");
        assert_eq!(value["description"], "The description.");
        assert_eq!(value["image"], "https://www.russellduhon.com/image.png");
        assert_eq!(value["author"]["name"], "Russell Duhon");
        assert_eq!(
            value["author"]["@id"],
            "https://www.russellduhon.com/#person"
        );
        assert_eq!(value["author"]["sameAs"][0], LINKEDIN_URL);
    }

    #[test]
    fn json_ld_omits_missing_optional_fields() {
        let serialized = blog_posting_json_ld(&post("bare", "Bare", None, None));
        let value: serde_json::Value =
            serde_json::from_str(&serialized).expect("JSON-LD should be valid JSON");
        assert!(value.get("description").is_none());
        assert!(value.get("image").is_none());
    }

    #[test]
    fn json_ld_never_contains_a_literal_angle_bracket() {
        let serialized = blog_posting_json_ld(&post(
            "sneaky",
            "About </script> tags",
            Some("Contains <b>markup</b>."),
            None,
        ));
        assert!(!serialized.contains('<'));
        let value: serde_json::Value =
            serde_json::from_str(&serialized).expect("escaped JSON-LD should still parse");
        assert_eq!(value["headline"], "About </script> tags");
    }
}
