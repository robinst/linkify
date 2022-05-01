use std::ops::Range;

pub trait Scanner {
    fn scan(
        &self,
        s: &str,
        trigger_index: usize,
        extract_wildcard_urls: bool,
    ) -> Option<Range<usize>>;
}
