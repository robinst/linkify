use std::ops::Range;

use crate::scanner::Scanner;

/// Scan for bug references such as `#12345`.
pub struct BugReferenceScanner;

impl Scanner for BugReferenceScanner {
    fn scan(&self, s: &str, hash: usize) -> Option<Range<usize>> {
        if !self.find_start(&s[..hash]) {
            return None;
        }

        let after_hash = hash + 1;
        let digits = s[after_hash..]
            .bytes()
            .take_while(|byte| byte.is_ascii_digit())
            .count();

        if digits == 0 {
            return None;
        }

        let end = after_hash + digits;
        if !self.find_end(&s[end..]) {
            return None;
        }

        Some(Range { start: hash, end })
    }
}

impl BugReferenceScanner {
    fn find_start(&self, s: &str) -> bool {
        match s.chars().next_back() {
            Some(c) => !Self::identifier_char(c) && c != '#',
            None => true,
        }
    }

    fn find_end(&self, s: &str) -> bool {
        match s.chars().next() {
            Some(c) => !Self::identifier_char(c),
            None => true,
        }
    }

    fn identifier_char(c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }
}
