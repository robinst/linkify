use memchr::memchr;
use memchr::memchr2;

use email::EmailScanner;
use scanner::Scanner;
use url::UrlScanner;

// TODO: Debug and other traits on the public types?

/// A link found in the input text.
pub struct Link<'t> {
    text: &'t str,
    start: usize,
    end: usize,
    kind: LinkKind,
}

impl<'t> Link<'t> {
    /// The start index of the link within the input text.
    #[inline]
    pub fn start(&self) -> usize {
        self.start
    }

    /// The end index of the link.
    #[inline]
    pub fn end(&self) -> usize {
        self.end
    }

    /// Get the link text as a `str`.
    #[inline]
    pub fn as_str(&self) -> &'t str {
        &self.text[self.start..self.end]
    }

    /// The type of the link.
    #[inline]
    pub fn kind(&self) -> &LinkKind {
        &self.kind
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum LinkKind {
    /// URL links like "http://example.org".
    Url,
    /// E-mail links like "foo@example.org"
    Email,
    /// Users should not exhaustively match this enum, because more link types
    /// may be added in the future.
    #[doc(hidden)]
    __Nonexhaustive,
}

/// A configured link finder.
pub struct LinkFinder {
    email: bool,
    email_domain_must_have_dot: bool,
    url: bool,
}

/// Iterator for finding links.
pub struct Links<'t> {
    text: &'t str,
    rewind: usize,

    trigger_finder: Box<Fn(&[u8]) -> Option<usize>>,
    email_scanner: EmailScanner,
    url_scanner: UrlScanner,
}

impl LinkFinder {
    /// Create a new link finder with the default options for finding all kinds
    /// of links.
    ///
    /// If you only want to find a certain kind of links, use the `kinds` method.
    pub fn new() -> LinkFinder {
        LinkFinder {
            email: true,
            email_domain_must_have_dot: true,
            url: true,
        }
    }

    /// Require the domain parts of email addresses to have at least one dot.
    /// Use `false` to also find addresses such as `root@localhost`.
    pub fn email_domain_must_have_dot(&mut self, value: bool) -> &mut LinkFinder {
        self.email_domain_must_have_dot = value;
        self
    }

    /// Restrict the kinds of links that should be found to the specified ones.
    pub fn kinds(&mut self, kinds: &[LinkKind]) -> &mut LinkFinder {
        self.email = false;
        self.url = false;
        for kind in kinds {
            match *kind {
                LinkKind::Email => { self.email = true }
                LinkKind::Url => { self.url = true }
                _ => {}
            }
        }
        self
    }

    /// Find links in the specified input text.
    ///
    /// Returns an `Iterator` which only scans when `next` is called (lazy).
    pub fn links<'t>(&self, text: &'t str) -> Links<'t> {
        Links::new(text, self.url, self.email, self.email_domain_must_have_dot)
    }
}

impl<'t> Links<'t> {
    fn new(text: &'t str, url: bool, email: bool, email_domain_must_have_dot: bool) -> Links<'t> {
        let url_scanner = UrlScanner {};
        let email_scanner = EmailScanner { domain_must_have_dot: email_domain_must_have_dot };
        let trigger_finder: Box<Fn(&[u8]) -> Option<usize>> = match (url, email) {
            (true, true) => Box::new(|s| memchr2(b':', b'@', s)),
            (true, false) => Box::new(|s| memchr(b':', s)),
            (false, true) => Box::new(|s| memchr(b'@', s)),
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
            let trigger = slice.as_bytes()[find_from + i];
            let (scanner, kind): (&Scanner, LinkKind) = match trigger {
                b':' => (&self.url_scanner, LinkKind::Url),
                b'@' => (&self.email_scanner, LinkKind::Email),
                _ => panic!("TODO"),
            };
            if let Some(range) = scanner.scan(slice, find_from + i) {
                let start = self.rewind + range.start;
                let end = self.rewind + range.end;
                self.rewind = end;
                return Some(Link {
                    text: &self.text,
                    start: start,
                    end: end,
                    kind: kind,
                });
            } else {
                // The scanner didn't find anything. But there could be more
                // trigger characters later, so continue the search.
                find_from += i + 1;
            }
        }

        return None;
    }
}
