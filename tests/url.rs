extern crate linkify;

mod common;

use crate::common::assert_linked_with;
use linkify::LinkFinder;

#[test]
fn no_links() {
    assert_not_linked("");
    assert_not_linked("foo");
    assert_not_linked(":");
    assert_not_linked("://");
    assert_not_linked(":::");
}

#[test]
fn schemes() {
    assert_not_linked("://foo");
    assert_not_linked("1://foo");
    assert_not_linked("123://foo");
    assert_not_linked("+://foo");
    assert_not_linked("-://foo");
    assert_not_linked(".://foo");
    assert_not_linked("1abc://foo");
    assert_linked("a://foo", "|a://foo|");
    assert_linked("a123://foo", "|a123://foo|");
    assert_linked("a123b://foo", "|a123b://foo|");
    assert_linked("a+b://foo", "|a+b://foo|");
    assert_linked("a-b://foo", "|a-b://foo|");
    assert_linked("a.b://foo", "|a.b://foo|");
    assert_linked("ABC://foo", "|ABC://foo|");
    assert_linked(".http://example.org/", ".|http://example.org/|");
    assert_linked("1.http://example.org/", "1.|http://example.org/|");
}

#[test]
fn authority() {
    assert_not_linked("ab://");
    assert_not_linked("http://");
    assert_not_linked("http:// ");
    assert_not_linked("\"http://\"");
    assert_not_linked("\"http://...\", ");

    assert_linked("http://a.", "|http://a|.");
}

#[test]
fn single_links() {
    assert_linked("ab://c", "|ab://c|");
    assert_linked("http://example.org/", "|http://example.org/|");
    assert_linked("http://example.org/123", "|http://example.org/123|");
    assert_linked(
        "http://example.org/?foo=test&bar=123",
        "|http://example.org/?foo=test&bar=123|",
    );
    assert_linked(
        "http://example.org/?foo=%20",
        "|http://example.org/?foo=%20|",
    );
    assert_linked("http://example.org/%3C", "|http://example.org/%3C|");
}

#[test]
fn space_characters_stop_url() {
    assert_linked("foo http://example.org/", "foo |http://example.org/|");
    assert_linked("http://example.org/ bar", "|http://example.org/| bar");
    assert_linked("http://example.org/\tbar", "|http://example.org/|\tbar");
    assert_linked("http://example.org/\nbar", "|http://example.org/|\nbar");
    assert_linked(
        "http://example.org/\u{0B}bar",
        "|http://example.org/|\u{0B}bar",
    );
    assert_linked(
        "http://example.org/\u{0C}bar",
        "|http://example.org/|\u{0C}bar",
    );
    assert_linked("http://example.org/\rbar", "|http://example.org/|\rbar");
}

#[test]
fn illegal_characters_stop_url() {
    assert_linked("http://example.org/<", "|http://example.org/|<");
    assert_linked("http://example.org/>", "|http://example.org/|>");
    assert_linked("http://example.org/<>", "|http://example.org/|<>");
    assert_linked("http://example.org/\u{00}", "|http://example.org/|\u{00}");
    assert_linked("http://example.org/\u{0E}", "|http://example.org/|\u{0E}");
    assert_linked("http://example.org/\u{7F}", "|http://example.org/|\u{7F}");
    assert_linked("http://example.org/\u{9F}", "|http://example.org/|\u{9F}");
}

#[test]
fn delimiter_at_end() {
    assert_linked("http://example.org/.", "|http://example.org/|.");
    assert_linked("http://example.org/..", "|http://example.org/|..");
    assert_linked("http://example.org/,", "|http://example.org/|,");
    assert_linked("http://example.org/:", "|http://example.org/|:");
    assert_linked("http://example.org/?", "|http://example.org/|?");
    assert_linked("http://example.org/!", "|http://example.org/|!");
    assert_linked("http://example.org/;", "|http://example.org/|;");
}

#[test]
fn matching_punctuation() {
    assert_linked("http://example.org/a(b)", "|http://example.org/a(b)|");
    assert_linked("http://example.org/a[b]", "|http://example.org/a[b]|");
    assert_linked("http://example.org/a{b}", "|http://example.org/a{b}|");
    assert_linked("http://example.org/a'b'", "|http://example.org/a'b'|");
    assert_linked("(http://example.org/)", "(|http://example.org/|)");
    assert_linked("[http://example.org/]", "[|http://example.org/|]");
    assert_linked("{http://example.org/}", "{|http://example.org/|}");
    assert_linked("\"http://example.org/\"", "\"|http://example.org/|\"");
    assert_linked("'http://example.org/'", "'|http://example.org/|'");
}

#[test]
fn matching_punctuation_tricky() {
    assert_linked("((http://example.org/))", "((|http://example.org/|))");
    assert_linked(
        "((http://example.org/a(b)))",
        "((|http://example.org/a(b)|))",
    );
    assert_linked("[(http://example.org/)]", "[(|http://example.org/|)]");
    assert_linked("(http://example.org/).", "(|http://example.org/|).");
    assert_linked("(http://example.org/.)", "(|http://example.org/|.)");
    assert_linked("http://example.org/>", "|http://example.org/|>");
    // not sure about these
    assert_linked("http://example.org/(", "|http://example.org/|(");
    assert_linked("http://example.org/(.", "|http://example.org/|(.");
    assert_linked("http://example.org/]()", "|http://example.org/|]()");
}

#[test]
fn single_quote() {
    assert_linked(
        "http://example.org/\'_(foo)",
        "|http://example.org/\'_(foo)|",
    );
    assert_linked(
        "http://example.org/\'_(foo)\'",
        "|http://example.org/\'_(foo)\'|",
    );
    assert_linked("http://example.org/\'\'", "|http://example.org/\'\'|");
    assert_linked("http://example.org/\'\'\'", "|http://example.org/\'\'|\'");
    assert_linked("http://example.org/\'.", "|http://example.org/|\'.");
    assert_linked("http://example.org/\'a", "|http://example.org/\'a|");
    assert_linked("http://example.org/it's", "|http://example.org/it's|");
}

#[test]
fn double_quote() {
    // " not allowed in URLs
    assert_linked("http://example.org/\"a", "|http://example.org/|\"a");
    assert_linked("http://example.org/\"a\"", "|http://example.org/|\"a\"");
}

#[test]
fn grave_quote() {
    // ` not allowed in URLs
    assert_linked("http://example.org/`a", "|http://example.org/|`a");
    assert_linked("http://example.org/`a`", "|http://example.org/|`a`");
}

#[test]
fn html() {
    assert_linked("http://example.org\">", "|http://example.org|\">");
    assert_linked("http://example.org'>", "|http://example.org|'>");
    assert_linked("http://example.org\"/>", "|http://example.org|\"/>");
    assert_linked("http://example.org'/>", "|http://example.org|'/>");
    assert_linked("http://example.org<p>", "|http://example.org|<p>");
    assert_linked("http://example.org</p>", "|http://example.org|</p>");
}

#[test]
fn css() {
    assert_linked("http://example.org\");", "|http://example.org|\");");
    assert_linked("http://example.org');", "|http://example.org|');");
}

#[test]
fn slash() {
    assert_linked("http://example.org/", "|http://example.org/|");
    assert_linked("http://example.org/a/", "|http://example.org/a/|");
    assert_linked("http://example.org//", "|http://example.org//|");
}

#[test]
fn multiple() {
    assert_linked(
        "http://one.org/ http://two.org/",
        "|http://one.org/| |http://two.org/|",
    );
    assert_linked(
        "http://one.org/ : http://two.org/",
        "|http://one.org/| : |http://two.org/|",
    );
    assert_linked(
        "(http://one.org/)(http://two.org/)",
        "(|http://one.org/|)(|http://two.org/|)",
    );
}

#[test]
fn international() {
    assert_linked(
        "http://üñîçøðé.com/ä",
        "|http://üñîçøðé.com/ä|",
    );
    assert_linked("http://example.org/\u{A1}", "|http://example.org/\u{A1}|");
    assert_linked("http://example.org/\u{A2}", "|http://example.org/\u{A2}|");
    assert_linked(
        "http://example.org/\u{1F600}",
        "|http://example.org/\u{1F600}|",
    );
    assert_linked("http://example.org/\u{A2}/", "|http://example.org/\u{A2}/|");
    assert_linked(
        "http://xn--c1h.example.com/",
        "|http://xn--c1h.example.com/|",
    );
}

#[test]
fn fuzz() {
    assert_not_linked("ab:/ϸ");
}

fn assert_not_linked(s: &str) {
    assert_linked(s, s);
}

fn assert_linked(input: &str, expected: &str) {
    let finder = LinkFinder::new();
    assert_linked_with(&finder, input, expected);
}
