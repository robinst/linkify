use std::ops::Range;

use memchr::memchr;
use memchr::memchr2;

use email::EmailScanner;
use scanner::Scanner;
use url::UrlScanner;

pub struct Link {
    pub range: Range<usize>,
    pub kind: LinkKind,
    _extensible: (),
}

pub enum LinkKind {
    /// URL links like "http://example.org".
    URL,
    /// E-mail links like "foo@example.org"
    EMAIL,
    /// Users should not exhaustively match this enum, because more link types may be added in the
    /// future.
    #[doc(hidden)]
    __Nonexhaustive,
}

pub struct LinkFinder {
    email: bool,
    email_domain_must_have_dot: bool,
    url: bool,
}

pub struct Links<'a> {
    input: &'a str,
    rewind: usize,

    trigger_finder: Box<Fn(&[u8]) -> Option<usize>>,
    email_scanner: EmailScanner,
    url_scanner: UrlScanner,
}

impl LinkFinder {
    pub fn new() -> LinkFinder {
        LinkFinder {
            email: true,
            email_domain_must_have_dot: true,
            url: true,
        }
    }

    pub fn email_domain_must_have_dot(&mut self, value: bool) -> &mut LinkFinder {
        self.email_domain_must_have_dot = value;
        self
    }

    // Not sure about this
    pub fn kinds(&mut self, kinds: &[LinkKind]) -> &mut LinkFinder {
        self.email = false;
        self.url = false;
        for kind in kinds {
            match *kind {
                LinkKind::EMAIL => { self.email = true }
                LinkKind::URL => { self.url = true }
                _ => {}
            }
        }
        self
    }

    pub fn links<'a>(&self, s: &'a str) -> Links<'a> {
        let email_scanner = EmailScanner { domain_must_have_dot: self.email_domain_must_have_dot };
        let url_scanner = UrlScanner {};
        let trigger_finder: Box<Fn(&[u8]) -> Option<usize>> = match (self.email, self.url) {
            (true, true) => Box::new(|s| memchr2(b'@', b':', s)),
            (true, false) => Box::new(|s| memchr(b'@', s)),
            (false, true) => Box::new(|s| memchr(b':', s)),
            (false, false) => Box::new(|_| None),
        };
        Links {
            input: s,
            rewind: 0,
            trigger_finder: trigger_finder,
            email_scanner: email_scanner,
            url_scanner: url_scanner,
        }
    }
}

impl<'a> Iterator for Links<'a> {
    type Item = Link;

    fn next(&mut self) -> Option<Link> {
        let slice = &self.input[self.rewind..];

        let mut find_from = 0;
        while let Some(i) = (self.trigger_finder)(slice[find_from..].as_bytes()) {
            let scanner: &Scanner = match slice.as_bytes()[find_from + i] {
                b'@' => &self.email_scanner,
                b':' => &self.url_scanner,
                _ => panic!("foo"),
            };
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

        return None;
    }
}
