use std::ops::Range;

use super::Scanner;

/// Scan for URLs starting from the trigger character ":", requires "://".
///
/// Based on RFC 3986.
pub struct UrlScanner {}

impl Scanner for UrlScanner {
    fn scan(&self, s: &str, colon: usize) -> Option<Range<usize>> {
        let length = s.len();
        let after_slash_slash = colon + 3;
        // Need at least one character for scheme, and one after '//'
        if colon > 0 && after_slash_slash < length {
            if &s[colon + 1..after_slash_slash] == "//" {
                if let Some(first) = self.find_first(s, colon) {
                    let last = self.find_last(s, after_slash_slash);
                    return Some(Range {
                        start: first,
                        end: last + 1,
                    });
                }
            }
        }
        return None;
    }
}

impl UrlScanner {
    // See "scheme" in RFC 3986
    fn find_first(&self, s: &str, colon: usize) -> Option<usize> {
        let mut first = None;
        let mut digit = None;
        for (i, c) in s[0..colon].char_indices().rev() {
            match c {
                'a'...'z' | 'A'...'Z' => first = Some(i),
                '0'...'9' => digit = Some(i),
                // scheme special
                '+' | '-' | '.' => {}
                _ => {
                    break;
                }
            }
        }

        // We don't want to extract "abc://foo" out of "1abc://foo".
        // ".abc://foo" and others are ok though, as they feel more like separators.
        if let Some(first) = first {
            if let Some(digit) = digit {
                if first > 0 && first - 1 == digit {
                    return None;
                }
            }
        }
        return first;
    }

    fn find_last(&self, s: &str, start: usize) -> usize {
        let mut round = 0;
        let mut square = 0;
        let mut curly = 0;
        let mut double_quote = false;
        let mut single_quote = false;

        let mut last = 0;

        for (i, c) in s[start..].char_indices() {
            match c {
                '\u{00}'...'\u{1F}' |
                ' ' |
                '<' |
                '>' |
                '\u{7F}'...'\u{9F}' => {
                    // These can never be part of an URL, so stop now. See RFC 3986 and RFC 3987.
                    // Some characters are not in the above list, even they are not in "unreserved"
                    // or "reserved":
                    //   '"', '\\', '^', '`', '{', '|', '}'
                    // The reason for this is that other link detectors also allow them. Also see
                    // below, we require the quote and the braces to be balanced.
                    break;
                }
                '?' | '!' | '.' | ',' | ':' | ';' => {
                    // These may be part of an URL but not at the end
                }
                '/' => {
                    // This may be part of an URL and at the end, but not if the previous character
                    // can't be the end of an URL
                    if last == i - 1 {
                        last = i;
                    }
                }
                '(' => round += 1,
                ')' => {
                    round -= 1;
                    if round >= 0 {
                        last = i;
                    } else {
                        // More closing than opening brackets, stop now
                        break;
                    }
                }
                '[' => {
                    // Allowed in IPv6 address host
                    square += 1;
                }
                ']' => {
                    // Allowed in IPv6 address host
                    square -= 1;
                    if square >= 0 {
                        last = i;
                    } else {
                        // More closing than opening brackets, stop now
                        break;
                    }
                }
                '{' => {
                    curly += 1;
                }
                '}' => {
                    curly -= 1;
                    if curly >= 0 {
                        last = i;
                    } else {
                        // More closing than opening brackets, stop now
                        break;
                    }
                }
                '"' => {
                    double_quote = !double_quote;
                    if !double_quote {
                        last = i;
                    }
                }
                '\'' => {
                    single_quote = !single_quote;
                    if !single_quote {
                        last = i;
                    }
                }
                _ => {
                    last = i;
                }
            }
        }

        start + last
    }
}
