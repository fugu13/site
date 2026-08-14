//! Syntax highlighting for fenced code blocks, via syntect.
//!
//! Output uses CSS classes prefixed with `hl-` (e.g. `hl-keyword`, `hl-string`,
//! `hl-comment`) rather than inline colors, so the palette lives entirely in the
//! stylesheet.

use std::sync::OnceLock;

use syntect::html::{ClassStyle, ClassedHTMLGenerator};
use syntect::parsing::{SyntaxReference, SyntaxSet};
use syntect::util::LinesWithEndings;

static SYNTAX_SET: OnceLock<SyntaxSet> = OnceLock::new();

fn syntax_set() -> &'static SyntaxSet {
    SYNTAX_SET.get_or_init(SyntaxSet::load_defaults_newlines)
}

/// Highlight `code` with `syntax`, returning `None` if any line fails to parse.
fn highlight_with_syntax(code: &str, syntax: &SyntaxReference) -> Option<String> {
    let mut generator = ClassedHTMLGenerator::new_with_class_style(
        syntax,
        syntax_set(),
        ClassStyle::SpacedPrefixed { prefix: "hl-" },
    );
    for line in LinesWithEndings::from(code) {
        generator
            .parse_html_for_line_which_includes_newline(line)
            .ok()?;
    }
    Some(generator.finalize())
}

/// Highlight a fenced code block tagged with `lang`, resolving common
/// aliases (e.g. "py" / "python", "rs" / "rust", and "sql" itself — extension
/// lookup happens before name lookup, so this covers plain-extension
/// languages like SQL too, with no per-language special-casing needed).
/// Returns `None` if the language isn't recognized or a line fails to parse,
/// leaving the escape-and-wrap fallback to the caller.
pub fn fenced(lang: &str, code: &str) -> Option<String> {
    let syntax = syntax_set().find_syntax_by_token(lang)?;
    highlight_with_syntax(code, syntax)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Strip HTML tags, leaving only text content, so highlighted output can
    /// be checked for the literal source text.
    fn strip_tags(html: &str) -> String {
        let mut result = String::new();
        let mut in_tag = false;
        for c in html.chars() {
            match c {
                '<' => in_tag = true,
                '>' => in_tag = false,
                _ if !in_tag => result.push(c),
                _ => {}
            }
        }
        result
    }

    #[test]
    fn fenced_python_has_classes_and_preserves_source() {
        let code = "def greet(name):\n    return \"hello \" + name\n";
        let html = fenced("python", code).expect("python should be a recognized language");
        assert!(html.contains("hl-"), "expected hl- classes in {html}");
        let text = strip_tags(&html);
        assert!(
            text.contains("greet"),
            "expected identifier to survive: {text}"
        );
        assert!(
            text.contains("hello"),
            "expected string literal to survive: {text}"
        );
    }

    #[test]
    fn fenced_rust_has_classes_and_preserves_source() {
        let code = "fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n";
        let html = fenced("rust", code).expect("rust should be a recognized language");
        assert!(html.contains("hl-"), "expected hl- classes in {html}");
        let text = strip_tags(&html);
        assert!(
            text.contains("add"),
            "expected identifier to survive: {text}"
        );
        assert!(
            text.contains("i32"),
            "expected type name to survive: {text}"
        );
    }

    #[test]
    fn fenced_unknown_language_returns_none() {
        assert!(fenced("not-a-real-language", "irrelevant").is_none());
    }

    #[test]
    fn fenced_sql_has_classes_and_preserves_source() {
        let code = "SELECT * FROM users WHERE id = 1;\n";
        let html = fenced("sql", code).expect("sql should be a recognized language");
        assert!(html.contains("hl-"), "expected hl- classes in {html}");
        let text = strip_tags(&html);
        assert!(
            text.contains("SELECT"),
            "expected keyword to survive: {text}"
        );
        assert!(
            text.contains("users"),
            "expected identifier to survive: {text}"
        );
    }
}
