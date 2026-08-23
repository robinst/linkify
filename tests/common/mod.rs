use linkify::{LinkFinder, LinkKind};

pub fn assert_linked_with(finder: &LinkFinder, input: &str, expected: &str) {
    let actual = show_links(input, finder);
    assert_eq!(actual, expected);
}

pub fn show_links(input: &str, finder: &LinkFinder) -> String {
    let mut result = String::new();

    for span in finder.spans(input) {
        if span.kind().is_some() {
            result.push('|');
            result.push_str(span.as_str());
            result.push('|');
        } else {
            result.push_str(span.as_str());
        }
    }
    result
}

/// Assert link without protocol
pub fn assert_urls_without_protocol(input: &str, expected: &str) {
    let mut finder = LinkFinder::new();
    finder.url_must_have_scheme(false);
    finder.kinds(&[LinkKind::Url]);
    assert_linked_with(&finder, input, expected);
}

/// Assert link with protocol
pub fn assert_linked(input: &str, expected: &str) {
    let finder = LinkFinder::new();
    assert_linked_with(&finder, input, expected);
}

pub fn assert_not_linked(s: &str) {
    assert_linked(s, s);
}
