use std::ops::Range;

use crate::scanner::Scanner;

const QUOTES: &[char] = &['\'', '\"'];

/// Scan for URLs starting from the trigger character ":", requires "://".
///
/// Based on RFC 3986.
pub struct UrlScanner {}

impl Scanner for UrlScanner {
    fn scan(&self, s: &str, colon: usize) -> Option<Range<usize>> {
        let after_slash_slash = colon + 3;
        // Need at least one character for scheme, and one after '//'
        if colon > 0 && after_slash_slash < s.len() && s[colon..].starts_with("://") {
            if let (Some(start), quote) = self.find_start(&s[0..colon]) {
                if let Some(end) = self.find_end(&s[after_slash_slash..], quote) {
                    let range = Range {
                        start,
                        end: after_slash_slash + end,
                    };
                    return Some(range);
                }
            }
        }
        None
    }
}

impl UrlScanner {
    // See "scheme" in RFC 3986
    fn find_start(&self, s: &str) -> (Option<usize>, Option<char>) {
        let mut first = None;
        let mut digit = None;
        let mut quote = None;
        for (i, c) in s.char_indices().rev() {
            match c {
                'a'..='z' | 'A'..='Z' => first = Some(i),
                '0'..='9' => digit = Some(i),
                // scheme special
                '+' | '-' | '.' => {}
                _ => {
                    // Check if there's a quote before the scheme,
                    // and stop once we encounter one of those quotes.
                    // https://github.com/robinst/linkify/issues/20
                    if QUOTES.contains(&c) {
                        quote = Some(c);
                    }
                    break;
                }
            }
        }

        // We don't want to extract "abc://foo" out of "1abc://foo".
        // ".abc://foo" and others are ok though, as they feel more like separators.
        if let Some(first) = first {
            if let Some(digit) = digit {
                // Comparing the byte indices with `- 1` is ok as scheme must be ASCII
                if first > 0 && first - 1 == digit {
                    return (None, quote);
                }
            }
        }
        (first, quote)
    }

    fn find_end(&self, s: &str, quote: Option<char>) -> Option<usize> {
        let mut round = 0;
        let mut square = 0;
        let mut curly = 0;
        let mut single_quote = false;

        let mut previous_can_be_last = true;
        let mut end = None;

        for (i, c) in s.char_indices() {
            let can_be_last = match c {
                '\u{00}'..='\u{1F}' | ' ' | '|' | '\"' | '<' | '>' | '`' | '\u{7F}'..='\u{9F}' => {
                    // These can never be part of an URL, so stop now. See RFC 3986 and RFC 3987.
                    // Some characters are not in the above list, even they are not in "unreserved"
                    // or "reserved":
                    //   '\\', '^', '{', '}'
                    // The reason for this is that other link detectors also allow them. Also see
                    // below, we require the braces to be balanced.
                    break;
                }
                '?' | '!' | '.' | ',' | ':' | ';' | '*' => {
                    // These may be part of an URL but not at the end. It's not that the spec
                    // doesn't allow them, but they are frequently used in plain text as delimiters
                    // where they're not meant to be part of the URL.
                    false
                }
                '/' => {
                    // This may be part of an URL and at the end, but not if the previous character
                    // can't be the end of an URL
                    previous_can_be_last
                }
                '(' => {
                    round += 1;
                    false
                }
                ')' => {
                    round -= 1;
                    if round < 0 {
                        // More closing than opening brackets, stop now
                        break;
                    }
                    true
                }
                '[' => {
                    // Allowed in IPv6 address host
                    square += 1;
                    false
                }
                ']' => {
                    // Allowed in IPv6 address host
                    square -= 1;
                    if square < 0 {
                        // More closing than opening brackets, stop now
                        break;
                    }
                    true
                }
                '{' => {
                    curly += 1;
                    false
                }
                '}' => {
                    curly -= 1;
                    if curly < 0 {
                        // More closing than opening brackets, stop now
                        break;
                    }
                    true
                }
                _ if Some(c) == quote => {
                    // Found matching quote from beginning of URL, stop now
                    break;
                }
                '\'' => {
                    single_quote = !single_quote;
                    // A single quote can only be the end of an URL if there's an even number
                    !single_quote
                }
                _ => true,
            };
            if can_be_last {
                end = Some(i + c.len_utf8());
            }
            previous_can_be_last = can_be_last;
        }

        end
    }
}
