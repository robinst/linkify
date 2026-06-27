mod common;

use crate::common::{assert_linked, assert_not_linked};

#[test]
fn ipv6_from_issue() {
    // test the literal links from the pr.
    assert_linked(
        "http://[FEDC:BA98:7654:3210:FEDC:BA98:7654:3210]:80/index.html",
        "|http://[FEDC:BA98:7654:3210:FEDC:BA98:7654:3210]:80/index.html|",
    );

    assert_linked(
        "http://[1080:0:0:0:8:800:200C:417A]/index.html",
        "|http://[1080:0:0:0:8:800:200C:417A]/index.html|",
    );

    assert_linked(
        "http://[3ffe:2a00:100:7031::1]",
        "|http://[3ffe:2a00:100:7031::1]|",
    );
    assert_linked(
        "http://[1080::8:800:200C:417A]/foo",
        "|http://[1080::8:800:200C:417A]/foo|",
    );

    assert_linked("http://[::192.9.5.5]/ipng", "|http://[::192.9.5.5]/ipng|");

    assert_linked(
        "http://[::FFFF:129.144.52.38]:80/index.html",
        "|http://[::FFFF:129.144.52.38]:80/index.html|",
    );

    assert_linked(
        "http://[2010:836B:4179::836B:4179]",
        "|http://[2010:836B:4179::836B:4179]|",
    );
}

#[test]
fn ipv6_full_uncompressed() {
    assert_linked(
        "http://[2001:0db8:0000:0000:0000:ff00:0042:8329]",
        "|http://[2001:0db8:0000:0000:0000:ff00:0042:8329]|",
    );
}

#[test]
fn ipv6_leading_zeros_omitted() {
    assert_linked(
        "https://[2001:db8:0:0:0:ff00:42:8329]",
        "|https://[2001:db8:0:0:0:ff00:42:8329]|",
    );
}

#[test]
fn ipv6_zero_comp_middle() {
    assert_linked(
        "https://[2001:db8::ff00:42:8329]",
        "|https://[2001:db8::ff00:42:8329]|",
    );
}

#[test]
fn ipv6_zero_comp_trailing() {
    assert_linked("http://[2001:db8:1234::]", "|http://[2001:db8:1234::]|");
}

#[test]
fn ipv6_zero_comp_leading() {
    assert_linked("http://[::1234:5678]", "|http://[::1234:5678]|");
}

#[test]
fn ipv6_zero_comp_loopback_comp() {
    assert_linked("https://[::1]", "|https://[::1]|");
}

#[test]
fn ipv6_unspecified_address() {
    assert_linked("http://[::]", "|http://[::]|");
}

// RFC 3986:

#[test]
fn ipv6_bracket_no_port() {
    assert_linked("http://[2001:db8::1]", "|http://[2001:db8::1]|");
}

#[test]
fn ipv6_bracket_with_port() {
    assert_linked("http://[2001:db8::1]:8080", "|http://[2001:db8::1]:8080|");
}

#[test]
fn ipv6_bracket() {
    assert_linked("http://[2001:db8::1]", "|http://[2001:db8::1]|");
}

#[test]
fn ipv6_bracket_slash() {
    assert_linked("https://[2001:db8::1]/", "|https://[2001:db8::1]/|");
}

#[test]
fn ipv6_bracket_path() {
    assert_linked(
        "http://[2001:db8::1]/index.html",
        "|http://[2001:db8::1]/index.html|",
    );
}

#[test]
fn ipv6_mixed_case() {
    assert_linked("http://[2001:DB8::A:b:C]", "|http://[2001:DB8::A:b:C]|");
}

#[test]
fn ipv6_max_hex() {
    assert_linked(
        "http://[ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff]",
        "|http://[ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff]|",
    );
}
// rfc 4007
#[test]
fn ipv6_zone_indices() {
    assert_linked(
        "http://[fe80::1ff:fe23:4567:890a%eth0]",
        "|http://[fe80::1ff:fe23:4567:890a%eth0]|",
    );
    assert_linked("http://[fe80::1%25]", "|http://[fe80::1%25]|");
}
// RFC 6874: IPv6 Zone Identifiers in URIs

#[test]
fn ipv6_zone_index_unencoded() {
    // Strictly speaking, unencoded '%' is not RFC compliant for URIs,
    // but it is extremely common in plain text and we want to capture it.
    assert_linked("http://[fe80::1%eth0]", "|http://[fe80::1%eth0]|");
}

#[test]
fn ipv6_zone_index_encoded() {
    // This is the strict RFC 6874 compliant format (%25 is the URL encoding for %)
    assert_linked("http://[fe80::1%25eth0]", "|http://[fe80::1%25eth0]|");
}

#[test]
fn ipv6_zone_index_with_port() {
    // Crucial: verify that the parser transitions correctly from the zone index
    // to the port after the closing bracket.
    assert_linked(
        "https://[fe80::1%eth0]:8080",
        "|https://[fe80::1%eth0]:8080|",
    );
    assert_linked(
        "https://[fe80::1%25eth0]:8080",
        "|https://[fe80::1%25eth0]:8080|",
    );
}

#[test]
fn ipv6_zone_index_with_path() {
    // Verify that the parser transitions correctly from the zone index to the path.
    assert_linked(
        "http://[fe80::1%eth0]/api/data",
        "|http://[fe80::1%eth0]/api/data|",
    );
    assert_linked(
        "http://[fe80::1%25eth0]/api/data",
        "|http://[fe80::1%25eth0]/api/data|",
    );
}

#[test]
fn ipv6_zone_index_with_port_and_path() {
    assert_linked(
        "http://[fe80::1%eth0]:443/api/data?query=1",
        "|http://[fe80::1%eth0]:443/api/data?query=1|",
    );
}

#[test]
fn ipv6_zone_index_ipv4_mapped() {
    // A rare but syntactically possible edge case
    assert_linked(
        "http://[::ffff:192.0.2.128%eth0]",
        "|http://[::ffff:192.0.2.128%eth0]|",
    );
}

#[test]
fn ipv6_zone_id_correct_place(){
	assert_linked("http://[::1%25eth0]", "|http://[::1%25eth0]|");

	// this is a malformed url- the %25eth0 should be in the brackets. a portion of the link is a valid url
	// so we correctly identify that and as for the part thats malformed thats not our job.
	assert_linked("http://[::1]%25eth0", "|http://[::1]|%25eth0");
}

#[test]
fn ipv6_ipv4_mapped() {
    assert_linked(
        "http://[::ffff:192.0.2.128]",
        "|http://[::ffff:192.0.2.128]|",
    );
    assert_linked(
        "http://[0:0:0:0:0:ffff:192.0.2.128]",
        "|http://[0:0:0:0:0:ffff:192.0.2.128]|",
    );
}

#[test]
fn ipv6_empty_brackets() {
    assert_not_linked("http://[]/");
}

#[test]
fn ipv6_unclosed_brackets() {
    assert_not_linked("http://[2001:db8::1/index.html");
}

#[test]
fn ipv6_non_hex() {
    assert_not_linked("http://[this-is-not-hex]/");
}
#[test]
fn ipv6_non_hex_but_looks_ip6y() {
    assert_not_linked("http://[this:is:not:hex]/");
}

#[test]
fn ipv6_nested_brackets() {
    assert_not_linked("http://[[::1]]/");
}
#[test]
fn ipv6_unclosed_bracket_at_eof() {
    assert_not_linked("http://[2001:");
}

#[test]
fn ipv6_unclosed_bracket_with_path() {
    assert_not_linked("http://[2001:/path");
}

#[test]
fn ipv6_with_complicated_surrounding_input() {
    assert_linked("So it's 8:10pm on or maybe its 1:21. time is an illusion. : anyway, [check out this link](http://[::])",
				  "So it's 8:10pm on or maybe its 1:21. time is an illusion. : anyway, [check out this link](|http://[::]|)")
}

#[test]
fn ipv6_with_too_many_colons() {
    // linkify does not do deep validation whether an ipv4 address is VALID, it extracts structurally valid boundaries.
    // so i'll do the same- otherwise we'd have to start counting colons and doing more complicated validations which...
    // would impact performance.
    assert_linked("http://[:::]", "|http://[:::]|");
    // this isn't a bug, its a feature. we are not validating the number of colons, this is heuristic link extractor,
    // not a strict URI validator. You can use this library to extract things that look like links and then use
    // rusts' standard url crate to validate the links and throw whatever errors you choose to.
    // counting the number of colons and doing more lookarounds than we're already doing would very likely impact performance.
    assert_linked(
        "http://[::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::]",
        "|http://[::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::]|",
    )
}
