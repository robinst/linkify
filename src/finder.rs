use memchr::memchr;
use memchr::memchr2;

use email::EmailScanner;
use scanner::Scanner;
use url::UrlScanner;

pub struct Link<'t> {
    text: &'t str,
    start: usize,
    end: usize,
    kind: LinkKind,
}

impl<'t> Link<'t> {
    #[inline]
    pub fn start(&self) -> usize {
        self.start
    }

    #[inline]
    pub fn end(&self) -> usize {
        self.end
    }

    #[inline]
    pub fn as_str(&self) -> &'t str {
        &self.text[self.start..self.end]
    }

    #[inline]
    pub fn kind(&self) -> &LinkKind {
        &self.kind
    }
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

pub struct Links<'t> {
    text: &'t str,
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

    pub fn links<'t>(&self, text: &'t str) -> Links<'t> {
        let email_scanner = EmailScanner { domain_must_have_dot: self.email_domain_must_have_dot };
        let url_scanner = UrlScanner {};
        let trigger_finder: Box<Fn(&[u8]) -> Option<usize>> = match (self.email, self.url) {
            (true, true) => Box::new(|s| memchr2(b'@', b':', s)),
            (true, false) => Box::new(|s| memchr(b'@', s)),
            (false, true) => Box::new(|s| memchr(b':', s)),
            (false, false) => Box::new(|_| None),
        };
        Links {
            text: text,
            rewind: 0,
            trigger_finder: trigger_finder,
            email_scanner: email_scanner,
            url_scanner: url_scanner,
        }
    }
}

impl<'t> Iterator for Links<'t> {
    type Item = Link<'t>;

    fn next(&mut self) -> Option<Link<'t>> {
        let slice = &self.text[self.rewind..];

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
                    text: &self.text,
                    start: start,
                    end: end,
                    kind: LinkKind::URL,
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
