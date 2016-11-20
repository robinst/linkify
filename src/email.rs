use std::ops::Range;

use super::Scanner;

/// Scan for email address starting from the trigger character "@".
///
/// Based on RFC 6531, but also scans invalid IDN. Doesn't match IP address in domain part or
/// quoting in local part.
pub struct EmailScanner {
    pub domain_must_have_dot: bool,
}

impl Scanner for EmailScanner {
    fn scan(&self, s: &str, at: usize) -> Option<Range<usize>> {
        if let Some(start) = self.find_start(s, at) {
            if let Some(end) = self.find_end(s, at) {
                return Some(Range {
                    start: start,
                    end: end,
                });
            }
        }
        return None;
    }
}

impl EmailScanner {
    // See "Local-part" in RFC 5321, plus extensions in RFC 6531
    fn find_start(&self, s: &str, at: usize) -> Option<usize> {
        let mut first = None;
        let mut atom_boundary = true;
        for (i, c) in s[0..at].char_indices().rev() {
            if Self::local_atom_allowed(c) {
                first = Some(i);
                atom_boundary = false;
            } else if c == '.' {
                if atom_boundary {
                    break;
                }
                atom_boundary = true;
            } else {
                break;
            }
        }
        first
    }

    // See "Domain" in RFC 5321, plus extension of "sub-domain" in RFC 6531
    fn find_end(&self, s: &str, at: usize) -> Option<usize> {
        let mut first_in_sub_domain = true;
        let mut can_end_sub_domain = false;
        let mut first_dot = None;
        let mut last = None;

        for (i, c) in s[at + 1..].char_indices() {
            if first_in_sub_domain {
                if Self::sub_domain_allowed(c) {
                    last = Some(i);
                    first_in_sub_domain = false;
                    can_end_sub_domain = true;
                } else {
                    break;
                }
            } else {
                if c == '.' {
                    if !can_end_sub_domain {
                        break;
                    }
                    first_in_sub_domain = true;
                    if first_dot.is_none() {
                        first_dot = Some(i);
                    }
                } else if c == '-' {
                    can_end_sub_domain = false;
                } else if Self::sub_domain_allowed(c) {
                    last = Some(i);
                    can_end_sub_domain = true;
                } else {
                    break;
                }
            }
        }

        if !self.domain_must_have_dot || first_dot.and_then(|d| last.map(|l| d < l)).unwrap_or(false) {
            last
        } else {
            None
        }
    }

    // See "Atom" in RFC 5321, "atext" in RFC 5322
    fn local_atom_allowed(c: char) -> bool {
        match c {
            'a' ... 'z' | 'A' ... 'Z' | '0' ... '9' | '!' | '#' | '$' | '%' | '&' | '\'' | '*' |
            '+' | '-' | '/' | '=' | '?' | '^' | '_' | '`' | '{' | '|' | '}' | '~' => true,
            _ => c >= '\u{80}',
        }
    }

    // See "sub-domain" in RFC 5321. Extension in RFC 6531 is simplified, this can also match invalid domains.
    fn sub_domain_allowed(c: char) -> bool {
        match c {
            'a' ... 'z' | 'A' ... 'Z' | '0' ... '9' => true,
            _ => c >= '\u{80}',
        }
    }
}
