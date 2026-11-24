mod common;

use crate::common::assert_linked_with;
use linkify::{LinkFinder, LinkKind};

#[test]
fn no_links() {
    assert_not_linked("");
    assert_not_linked("#");
    assert_not_linked("#abc");
    assert_not_linked("foo#123");
    assert_not_linked("#123abc");
    assert_not_linked("##123");
}

#[test]
fn simple() {
    assert_linked("#12345", "|#12345|");
    assert_linked("See #12345.", "See |#12345|.");
    assert_linked("(#12345)", "(|#12345|)");
}

#[test]
fn multiple() {
    assert_linked("#1 and #22", "|#1| and |#22|");
}

#[test]
fn kind_filtering() {
    let mut finder = LinkFinder::new();
    finder.kinds(&[LinkKind::BugReference]);

    assert_linked_with(
        &finder,
        "#12 foo@example.com http://example.com",
        "|#12| foo@example.com http://example.com",
    );
}

#[test]
fn default_finder_includes_bug_references() {
    let finder = LinkFinder::new();
    let link = finder.links("fixed in #12345").next().unwrap();

    assert_eq!(link.kind(), &LinkKind::BugReference);
    assert_eq!(link.as_str(), "#12345");
    assert_eq!(link.href(), "#12345");
}

#[test]
fn href_uses_configured_prefix() {
    let mut finder = LinkFinder::new();
    finder.bug_reference_prefix("https://example.org/bugs/");

    let link = finder.links("fixed in #12345").next().unwrap();

    assert_eq!(link.kind(), &LinkKind::BugReference);
    assert_eq!(link.as_str(), "#12345");
    assert_eq!(link.href(), "https://example.org/bugs/12345");
}

fn assert_not_linked(s: &str) {
    let mut finder = LinkFinder::new();
    finder.kinds(&[LinkKind::BugReference]);
    let result = finder.links(s);
    assert_eq!(result.count(), 0, "expected no links in {:?}", s);
}

fn assert_linked(input: &str, expected: &str) {
    let mut finder = LinkFinder::new();
    finder.kinds(&[LinkKind::BugReference]);
    assert_linked_with(&finder, input, expected);
}
