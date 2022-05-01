use std::ops::Range;

use crate::email;
use crate::scanner::Scanner;

/// Minimum valid URL length
///
/// The shortest valid URL (without a scheme) might be g.cn (Google China),
/// which consists of four characters.
/// We set this as a lower threshold for parsing URLs from plaintext
/// to avoid false-positives and as a slight performance optimization.
/// This threshold might be adjusted in the future.
const MIN_URL_LENGTH: usize = 4;

const QUOTES: &[char] = &['\'', '\"'];

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
    fn scan(&self, s: &str, separator: usize, extract_wildcard_urls: bool) -> Option<Range<usize>> {
        // There must be something before separator for scheme or host
        if separator == 0 {
            return None;
        }

        if s.len() < MIN_URL_LENGTH {
            // URL shorter than threshold; skip parsing
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

        // When scanning a dot, for URLs without a scheme, do some additional checks
        if !is_slash_slash {
            // The TLD must be at least 2 characters long
            let end = &s[after_separator..];
            let after_last_dot = end
                .split('/')
                .next()
                .unwrap()
                .rfind('.')
                .map(|pos| pos + 1)
                .unwrap_or(0);
            let tld_too_short = end[after_last_dot..]
                .chars()
                .take_while(|c| c.is_ascii_alphabetic())
                .take(2)
                .count()
                < 2;
            if tld_too_short {
                return None;
            }

            // If this is an email address, don't scan it as URL
            if email::is_mail(&s[after_separator..]) {
                return None;
            }
        }

        // Need at least one character for scheme, and one after '//'
        if after_separator < s.len() {
            if let (Some(start), quote) = self.find_start(&s[0..separator], is_slash_slash) {
                if let Some(end) =
                    self.find_end(&s[after_separator..], quote, extract_wildcard_urls)
                {
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
    // true because the URL will have a scheme for sure. If searching before the `.` separator, it
    // should be `false` as we might search over the scheme definition for the scheme being optional.
    // See "scheme" in RFC 3986
    fn find_start(&self, s: &str, mut has_scheme: bool) -> (Option<usize>, Option<char>) {
        let mut first = None;
        let mut special = None;
        let mut quote = None;
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
                '@' => return (None, None),
                c if QUOTES.contains(&c) => {
                    // Check if there's a quote before the scheme,
                    // and stop once we encounter one of those quotes.
                    // https://github.com/robinst/linkify/issues/20
                    quote = Some(c);
                }
                c if !has_scheme && !matches!(c, '(' | ')' | '[' | ']' | '{' | '}' | ' ') => {
                    // Detect the start for links using unicode when having links without a scheme,
                    // then looking for ASCII alpha characters is not enough
                    first = Some(i);
                }
                _ => break,
            }
        }

        // We don't want to extract "abc://foo" out of "1abc://foo".
        // ".abc://foo" and others are ok though, as they feel more like separators.
        if let Some(first) = first {
            if let Some(special) = special {
                // Comparing the byte indices with `- 1` is ok as scheme must be ASCII
                if first > 0 && first - 1 == special {
                    return (None, quote);
                }
            }
        }
        (first, quote)
    }

    fn find_end(&self, s: &str, quote: Option<char>, extract_wildcard_urls: bool) -> Option<usize> {
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
                    if !extract_wildcard_urls {
                        // Stop and return early in case wildcard URLs like `http://*.example.com`
                        // shall not be extracted.
                        break;
                    }
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
