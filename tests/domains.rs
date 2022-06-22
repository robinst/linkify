//! This is called domains for familiarity but it's about the authority part of URLs as defined in
//! https://datatracker.ietf.org/doc/html/rfc3986#section-3.2
//!
//! ```
//! authority   = [ userinfo "@" ] host [ ":" port ]
//!
//!
//! userinfo    = *( unreserved / pct-encoded / sub-delims / ":" )
//!
//! host        = IP-literal / IPv4address / reg-name
//!
//! IP-literal = "[" ( IPv6address / IPvFuture  ) "]"
//!
//! IPv4address = dec-octet "." dec-octet "." dec-octet "." dec-octet
//!
//! reg-name    = *( unreserved / pct-encoded / sub-delims )
//!
//!
//! unreserved  = ALPHA / DIGIT / "-" / "." / "_" / "~"
//!
//! sub-delims  = "!" / "$" / "&" / "'" / "(" / ")" / "*" / "+" / "," / ";" / "="
//! ```

mod common;

use crate::common::assert_linked_with;
use linkify::LinkFinder;

#[test]
fn domain_valid() {
    // assert_linked("9292.nl", "|9292.nl|");
    // assert_linked("a12.b-c.com", "|a12.b-c.com|");
    // Trailing dot allowed
    assert_linked("https://example.com./test", "|https://example.com./test|");
}

#[test]
fn domain_with_userinfo() {
    assert_linked(
        "https://user:pass@example.com/",
        "|https://user:pass@example.com/|",
    );
    assert_linked(
        "https://user:-.!$@example.com/",
        "|https://user:-.!$@example.com/|",
    );

    // Can't have another @
    assert_not_linked("https://user:pass@ex@mple.com/");
}

#[test]
fn domain_with_port() {
    assert_linked("https://localhost:8080/", "|https://localhost:8080/|");
}

#[test]
fn domain_with_userinfo_and_port() {
    assert_linked(
        "https://user:pass@example.com:8080/hi",
        "|https://user:pass@example.com:8080/hi|",
    );
}

#[test]
fn domain_ipv6() {
    assert_linked("https://[dontcare]/", "|https://[dontcare]/|");
}

#[test]
fn domain_ipv4() {
    assert_linked("https://127.0.0.1/", "|https://127.0.0.1/|");
}

#[test]
fn domain_labels_cant_be_empty() {
    assert_not_linked("www.example..com");
}

#[test]
fn domain_labels_cant_start_with_hyphen() {
    assert_not_linked("-a.com");
    // assert_not_linked("https://a.-b.com");
}

#[test]
fn domain_labels_cant_end_with_hyphen() {
    assert_not_linked("a-.com");
    assert_not_linked("a.b-.com");

    // assert_not_linked("https://a.b-.com");
    // Could also argue that it should be linked without the "-" (like e.g. ";" at the end)
    // assert_not_linked("https://example.org-");
}

#[test]
fn domain_cant_contain_at() {
    // Looks like an email but was recognized as a schemeless link before.
    // assert_not_linked("example.com@about");
    // As part of path it's ok.
    assert_linked("example.com/@about", "|example.com/@about|");
    // assert_linked("https://example.com/@about", "|https://example.com/@about|");
}

#[test]
fn domain_cant_end_numeric() {
    assert_not_linked("info@v1.1.1");
}

#[test]
fn no_authority_part() {
    assert_linked("file:///", "|file:///|");
    assert_linked("file:///home/foo", "|file:///home/foo|");
}

#[test]
fn authority_thats_not_domain() {
    // Not valid according to DNS but we should allow it for other schemes (or all, not sure).
    assert_linked("facetime://+19995551234", "|facetime://+19995551234|");
}

#[test]
fn without_scheme_should_stop() {
    // assert_linked("ab/example.com", "ab/|example.com|");
    // This is not a valid scheme. Even the schemeless parser should not accept it, nor extract
    // only example.com out of it.
    assert_not_linked("1abc://example.com");
}

fn assert_linked(input: &str, expected: &str) {
    let mut finder = LinkFinder::new();
    finder.url_must_have_scheme(false);

    assert_linked_with(&finder, input, expected);
}

fn assert_not_linked(s: &str) {
    assert_linked(s, s);
}
