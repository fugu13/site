//! Shared escaping for XML element text and HTML/XML attribute values.

use std::borrow::Cow;

/// Escape the five XML/HTML-significant characters.
///
/// Borrows the input unchanged when it contains none. Safe for element text
/// and for double- or single-quoted attribute values, in both XML and HTML
/// output.
pub fn xml_escape(text: &str) -> Cow<'_, str> {
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

    #[test]
    fn clean_text_is_borrowed_unchanged() {
        assert!(matches!(
            xml_escape("plain text"),
            Cow::Borrowed("plain text")
        ));
    }

    #[test]
    fn all_five_significant_characters_are_escaped() {
        assert_eq!(
            xml_escape(r#"<a href="x">&'</a>"#),
            "&lt;a href=&quot;x&quot;&gt;&amp;&apos;&lt;/a&gt;"
        );
    }
}
