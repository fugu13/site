//! Markdown-to-HTML rendering for post bodies.
//!
//! Deliberately mirrors a legacy CommonMark-only renderer: `Options::empty()`
//! leaves tables, strikethrough, and (critically) smart punctuation disabled,
//! so straight quotes in source posts are never rewritten to curly ones.
//!
//! Fenced code blocks are intercepted and re-emitted as raw HTML, routed
//! through [`crate::highlight`] for syntax coloring, and wrapped in a
//! focusable scroll region so keyboard users can reach horizontal overflow.
//!
//! A fence's info string is `language` optionally followed by a short label,
//! e.g. ```` ```python The complete test file ````. The first word picks the
//! syntax highlighting; the rest becomes the scroll region's `aria-label`, so
//! a screen reader announces what each code sample is rather than a generic
//! "Code sample" (the fallback when no label is written).

use pulldown_cmark::{CodeBlockKind, CowStr, Event, Options, Parser, Tag, TagEnd};

/// Render Markdown `source` to an HTML string.
pub fn to_html(source: &str) -> String {
    let parser = Parser::new_ext(source, Options::empty());

    let mut events = Vec::new();
    let mut in_fenced_block = false;
    let mut fence_info = String::new();
    let mut code_text = String::new();

    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(info))) => {
                in_fenced_block = true;
                fence_info = info.to_string();
                code_text.clear();
            }
            Event::Text(text) if in_fenced_block => {
                code_text.push_str(&text);
            }
            Event::End(TagEnd::CodeBlock) if in_fenced_block => {
                in_fenced_block = false;
                let html = render_fenced_block(&fence_info, &code_text);
                events.push(Event::Html(CowStr::from(html)));
            }
            other => events.push(other),
        }
    }

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, events.into_iter());
    html_output
}

/// Render one fenced code block's inner HTML, wrap it in `<pre><code>`, and
/// wrap that in a keyboard-focusable scroll region whose `aria-label` is the
/// fence's label, or "Code sample" when none was written. Language tagging:
/// no language at all skips the highlighter entirely and omits the `class`
/// attribute; a present language always gets the `class`, whether or not
/// highlighting actually succeeded.
fn render_fenced_block(info: &str, code: &str) -> String {
    let info = info.trim();
    let (lang, label) = info.split_once(char::is_whitespace).unwrap_or((info, ""));

    let pre = if lang.is_empty() {
        format!("<pre><code>{}</code></pre>", escape_html(code))
    } else {
        let inner = crate::highlight::fenced(lang, code).unwrap_or_else(|| escape_html(code));
        format!("<pre><code class=\"language-{lang}\">{inner}</code></pre>")
    };

    let label = match label.trim() {
        "" => std::borrow::Cow::Borrowed("Code sample"),
        label => crate::escape::xml_escape(label),
    };
    format!("<div role=\"region\" tabindex=\"0\" aria-label=\"{label}\">{pre}</div>")
}

/// Escape the three HTML-significant characters, `&` first so escaping the
/// other two doesn't double-escape the ampersands they introduce. Code
/// bodies nearly always contain these, so unlike [`crate::escape::xml_escape`]
/// this allocates unconditionally rather than checking first.
fn escape_html(code: &str) -> String {
    code.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_paragraph_renders_as_commonmark() {
        let html = to_html("Hello *world*.");
        assert_eq!(html.trim(), "<p>Hello <em>world</em>.</p>");
    }

    #[test]
    fn straight_quotes_are_not_curled() {
        // Smart punctuation must stay off: this would otherwise fail because
        // pulldown-cmark's smartypants would produce curly quotes.
        let html = to_html("She said \"hi\" and it's fine.");
        assert!(html.contains('"'), "expected straight quotes: {html}");
        assert!(!html.contains('\u{201c}'), "found curly left quote: {html}");
        assert!(!html.contains('\u{2019}'), "found curly apostrophe: {html}");
    }

    #[test]
    fn tables_are_not_enabled() {
        // With Options::empty(), a GFM table is just an unrecognized paragraph
        // of pipe-delimited text, not a <table>.
        let html = to_html("| a | b |\n| - | - |\n| 1 | 2 |\n");
        assert!(
            !html.contains("<table"),
            "tables should be disabled: {html}"
        );
    }

    #[test]
    fn fenced_sql_block_is_highlighted_with_language_class() {
        let html = to_html("```sql\nSELECT * FROM users;\n```\n");
        assert!(
            html.contains("<pre><code class=\"language-sql\">"),
            "expected language-tagged pre/code: {html}"
        );
        assert!(
            html.contains("SELECT"),
            "expected source text to survive: {html}"
        );
    }

    #[test]
    fn fenced_python_block_is_highlighted_with_language_class() {
        let html = to_html("```python\ndef f():\n    pass\n```\n");
        assert!(
            html.contains("<pre><code class=\"language-python\">"),
            "expected language-tagged pre/code: {html}"
        );
        assert!(html.contains("hl-"), "expected highlighted classes: {html}");
    }

    #[test]
    fn fenced_block_with_unknown_language_falls_back_to_escaped_plain_text() {
        let html = to_html("```not-a-real-language\n<tag> & things\n```\n");
        assert!(
            html.contains("<pre><code class=\"language-not-a-real-language\">"),
            "expected the class to be kept even when highlighting fails: {html}"
        );
        assert!(
            html.contains("&lt;tag&gt; &amp; things"),
            "expected escaped fallback: {html}"
        );
    }

    #[test]
    fn fenced_block_with_no_language_has_no_class_and_is_never_highlighted() {
        let html = to_html("```\n<tag> & things\n```\n");
        assert!(
            html.contains("<pre><code>&lt;tag&gt; &amp; things\n</code></pre>"),
            "expected bare pre/code with escaped text: {html}"
        );
        assert!(
            !html.contains("class="),
            "no language tag means no class at all: {html}"
        );
    }

    #[test]
    fn soft_line_break_renders_as_whitespace() {
        // CommonMark soft breaks (a newline inside a paragraph, no trailing
        // double-space/backslash) become whitespace in the HTML output,
        // which browsers collapse to a single space in normal flow. The
        // pre-migration site glued a handful of specific word pairs together
        // instead, from a bug in its own renderer (see BUGHISTORY.md) — that
        // bug is not reproduced here; this renderer stays a correct
        // CommonMark implementation for every post.
        let html = to_html("implement in-place traversal for a\nbinary tree.");
        assert!(
            !html.contains("abinary"),
            "words must not be joined by a dropped soft break: {html}"
        );
    }

    #[test]
    fn fence_label_becomes_the_regions_aria_label() {
        let html = to_html("```python The complete test file\npass\n```\n");
        assert!(
            html.contains("aria-label=\"The complete test file\""),
            "expected the fence label as aria-label: {html}"
        );
        assert!(
            html.contains("<pre><code class=\"language-python\">"),
            "the first info-string word must still pick the language: {html}"
        );
    }

    #[test]
    fn fence_label_is_attribute_escaped() {
        let html = to_html("```python A \"quoted\" <label> & more\npass\n```\n");
        assert!(
            html.contains("aria-label=\"A &quot;quoted&quot; &lt;label&gt; &amp; more\""),
            "expected an attribute-escaped label: {html}"
        );
    }

    #[test]
    fn unlabeled_fence_keeps_the_generic_label() {
        let html = to_html("```python\npass\n```\n");
        assert!(
            html.contains("aria-label=\"Code sample\""),
            "expected the generic fallback label: {html}"
        );
    }

    #[test]
    fn fenced_block_is_wrapped_in_a_focusable_scroll_region() {
        // A11Y-002: a keyboard-only user needs to be able to scroll a
        // horizontally overflowing code block into view.
        let html = to_html("```\nplain\n```\n");
        assert!(
            html.contains("<div role=\"region\" tabindex=\"0\""),
            "fenced blocks must be wrapped in a focusable scroll region: {html}"
        );
        assert!(
            html.contains("<pre><code>plain\n</code></pre></div>"),
            "the pre/code element itself must stay unwrapped: {html}"
        );
    }
}
