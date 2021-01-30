use std::ops::Range;

use crate::scanner::Scanner;

/// Scan for URLs starting from the trigger character ":", requires "://".
///
/// Based on RFC 3986.
pub struct UrlScanner;

impl Scanner for UrlScanner {
    /// Scan for an URL at the given separator index in the string.
    ///
    /// The kind of separator that was used (`://` vs `.`) has effect on whether URLs with no
    /// schemes are found.
    ///
    /// Returns `None` if none was found, or if an invalid separator index was given.
    fn scan(&self, s: &str, separator: usize) -> Option<Range<usize>> {
        // There must be something before separator for scheme or host
        if separator == 0 {
            return None;
        }

        // Detect used separator, being `://` or `.`
        let (is_slash_slash, separator_len) = if s[separator..].starts_with("://") {
            (true, "://".len())
        } else if s[separator..].starts_with('.') {
            (false, ".".len())
        } else {
            return None;
        };
        let after_separator = separator + separator_len;

        if after_separator < s.len() {
            if let Some(start) = self.find_start(&s[0..separator], is_slash_slash) {
                if let Some(end) = self.find_end(&s[after_separator..]) {
                    let range = Range {
                        start,
                        end: after_separator + end,
                    };
                    return Some(range);
                }
            }
        }
        None
    }
}

impl UrlScanner {
    // For URL searching starting before the `://` separator, the `has_scheme` parameter should be
    // true because the URL will have a scheme for sure. If seraching before the `.` separator, it
    // should be `false` as we might search over the scheme definition for the scheme being optional.
    // See "scheme" in RFC 3986
    fn find_start(&self, s: &str, mut has_scheme: bool) -> Option<usize> {
        let mut first = None;
        let mut special = None;
        for (i, c) in s.char_indices().rev() {
            match c {
                'a'..='z' | 'A'..='Z' => first = Some(i),
                '0'..='9' => special = Some(i),
                '/' if !has_scheme => special = Some(i),
                ':' if !has_scheme => {
                    has_scheme = true;
                    special = Some(i)
                }
                '+' | '-' | '.' => {}
                _ => {
                    break;
                }
            }
        }

        // We don't want to extract "abc://foo" out of "1abc://foo".
        // ".abc://foo" and others are ok though, as they feel more like separators.
        if let Some(first) = first {
            if let Some(special) = special {
                // Comparing the byte indices with `- 1` is ok as scheme must be ASCII
                if first > 0 && first - 1 == special {
                    return None;
                }
            }
        }
        first
    }

    fn find_end(&self, s: &str) -> Option<usize> {
        let mut round = 0;
        let mut square = 0;
        let mut curly = 0;
        let mut single_quote = false;

        let mut previous_can_be_last = true;
        let mut end = None;

        for (i, c) in s.char_indices() {
            let can_be_last = match c {
                '\u{00}'..='\u{1F}' | ' ' | '\"' | '<' | '>' | '`' | '\u{7F}'..='\u{9F}' => {
                    // These can never be part of an URL, so stop now. See RFC 3986 and RFC 3987.
                    // Some characters are not in the above list, even they are not in "unreserved"
                    // or "reserved":
                    //   '\\', '^', '{', '|', '}'
                    // The reason for this is that other link detectors also allow them. Also see
                    // below, we require the braces to be balanced.
                    break;
                }
                '?' | '!' | '.' | ',' | ':' | ';' => {
                    // These may be part of an URL but not at the end
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
