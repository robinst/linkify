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

extern crate memchr;

mod email;
mod url;

use std::ops::Range;

use memchr::memchr;

use url::UrlScanner;

pub struct Link {
    pub range: Range<usize>,
    pub kind: LinkKind,
    _extensible: (),
}

pub enum LinkKind {
    /// URL links like "http://example.org".
    URL,
    /// Users should not exhaustively match this enum, because more link types may be added in the
    /// future.
    #[doc(hidden)]
    __Nonexhaustive,
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
        let slice = &self.input[self.rewind..];

        let mut find_from = 0;
        while let Some(i) = memchr(b':', slice[find_from..].as_bytes()) {
            if let Some(ref scanner) = self.url_scanner {
                if let Some(range) = scanner.scan(slice, find_from + i) {
                    let start = self.rewind + range.start;
                    let end = self.rewind + range.end;
                    self.rewind = end;
                    return Some(Link {
                        range: Range {
                            start: start,
                            end: end,
                        },
                        kind: LinkKind::URL,
                        _extensible: (),
                    });
                } else {
                    // The scanner didn't find anything. But there could be more trigger characters
                    // later, so continue the search.
                    find_from += i + 1;
                }
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
