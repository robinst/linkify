//! TODO.
//!
//! ```
//! use linkify::LinkFinder;
//!
//! let input = "Have you seen http://example.org?";
//! let finder = LinkFinder::new();
//! let links: Vec<_> = finder.find(input).collect();
//!
//! assert_eq!(1, links.len());
//! let link = &links[0];
//!
//! assert_eq!("http://example.org", &input[link.range.clone()]);
//! ```

use std::ops::Range;

use self::url::UrlScanner;

mod url;

pub struct Link {
    pub range: Range<usize>,
    // TODO: Make enum? Allow extensibility?
}

pub struct LinkFinder {}

pub struct Links<'a> {
    input: &'a str,
    rewind: usize,
    url_scanner: Option<UrlScanner>,
}

impl<'a> Iterator for Links<'a> {
    type Item = Link;

    fn next(&mut self) -> Option<Link> {
        let buffer = &self.input[self.rewind..];
        for (i, c) in buffer.char_indices() {
            match c {
                ':' => {
                    if let Some(ref scanner) = self.url_scanner {
                        if let Some(range) = scanner.scan(buffer, i) {
                            let start = self.rewind + range.start;
                            let end = self.rewind + range.end;
                            self.rewind = end;
                            return Some(Link {
                                range: Range {
                                    start: start,
                                    end: end,
                                },
                            });
                        }
                    }
                }
                _ => {}
            }
        }
        return None;
    }
}

trait Scanner {
    fn scan(&self, s: &str, trigger_index: usize) -> Option<Range<usize>>;
}

impl LinkFinder {
    pub fn new() -> LinkFinder {
        LinkFinder {}
    }

    pub fn find<'a>(&self, s: &'a str) -> Links<'a> {
        Links {
            input: s,
            rewind: 0,
            // TODO: Configuration (builder style)
            url_scanner: Some(UrlScanner {}),
        }
    }
}
