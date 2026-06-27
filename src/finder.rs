use std::fmt;
use std::iter::Peekable;

use crate::bug::BugReferenceScanner;
use crate::email::EmailScanner;
use crate::scanner::Scanner;
use crate::url::{DomainScanner, UrlScanner};

/// A link found in the input text.
#[derive(Debug)]
pub struct Link<'t> {
    text: &'t str,
    start: usize,
    end: usize,
    href: Option<String>,
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

    /// Get the link destination.
    ///
    /// For most links this is the same as `as_str()`. Bug references return the
    /// configured prefix plus the bug number when a bug reference prefix was set
    /// on the `LinkFinder`.
    #[inline]
    pub fn href(&self) -> &str {
        self.href.as_deref().unwrap_or_else(|| self.as_str())
    }

    /// The type of the link.
    #[inline]
    pub fn kind(&self) -> &LinkKind {
        &self.kind
    }
}

/// The type of link that was found.
#[derive(Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum LinkKind {
    /// URL links like "http://example.org".
    Url,
    /// E-mail links like "foo@example.org"
    Email,
    /// Bug references like "#12345".
    BugReference,
}

/// Span within the input text.
///
/// A span represents a substring of the input text,
/// which can either be a link, or plain text.
#[derive(Debug)]
pub struct Span<'t> {
    text: &'t str,
    start: usize,
    end: usize,
    kind: Option<LinkKind>,
}

impl<'t> Span<'t> {
    /// The start index of the span within the input text.
    #[inline]
    pub fn start(&self) -> usize {
        self.start
    }

    /// The end index of the span.
    #[inline]
    pub fn end(&self) -> usize {
        self.end
    }

    /// Get the span text as a `str`.
    #[inline]
    pub fn as_str(&self) -> &'t str {
        &self.text[self.start..self.end]
    }

    /// The type of link included in the span, if any.
    ///
    /// Returns `None` if the span represents plain text.
    #[inline]
    pub fn kind(&self) -> Option<&LinkKind> {
        self.kind.as_ref()
    }
}

/// A configured link finder.
#[derive(Debug)]
pub struct LinkFinder {
    bug_reference: bool,
    bug_reference_prefix: Option<String>,
    email: bool,
    email_domain_must_have_dot: bool,
    url: bool,
    url_must_have_scheme: bool,
    url_can_be_iri: bool,
}

type TriggerFinder = dyn Fn(&[u8]) -> Option<usize>;

/// Iterator for finding links.
pub struct Links<'t> {
    text: &'t str,
    rewind: usize,
    bug_reference_prefix: Option<String>,

    trigger_finder: Box<TriggerFinder>,
    bug_reference_scanner: BugReferenceScanner,
    email_scanner: EmailScanner,
    url_scanner: UrlScanner,
    domain_scanner: DomainScanner,
}

/// Iterator over spans.
pub struct Spans<'t> {
    text: &'t str,
    position: usize,
    links: Peekable<Links<'t>>,
}

impl LinkFinder {
    /// Create a new link finder with the default options for finding all kinds
    /// of links.
    ///
    /// If you only want to find a certain kind of links, use the `kinds` method.
    pub fn new() -> LinkFinder {
        LinkFinder {
            bug_reference: true,
            bug_reference_prefix: None,
            email: true,
            email_domain_must_have_dot: true,
            url: true,
            url_must_have_scheme: true,
            url_can_be_iri: true,
        }
    }

    /// Require the domain parts of email addresses to have at least one dot.
    /// Use `false` to also find addresses such as `root@localhost`.
    pub fn email_domain_must_have_dot(&mut self, value: bool) -> &mut LinkFinder {
        self.email_domain_must_have_dot = value;
        self
    }

    /// Set whether URLs must have a scheme, defaults to `true`.
    ///
    /// By default only URLs having a scheme defined are found.
    /// By setting this to `false` you make the scheme of URLs optional, to also find URLs like
    /// `example.org`. For some URLs the used scheme is important, and making the scheme optional
    /// may lead to finding a lot of false positive URLs.
    pub fn url_must_have_scheme(&mut self, url_must_have_scheme: bool) -> &mut LinkFinder {
        self.url_must_have_scheme = url_must_have_scheme;
        self
    }

    /// Sets whether URLs can be IRI according to RFC-3987.
    /// The default is `true`.
    /// Setting it to `false` means domains can contain ASCII characters only.
    pub fn url_can_be_iri(&mut self, url_can_be_iri: bool) -> &mut LinkFinder {
        self.url_can_be_iri = url_can_be_iri;
        self
    }

    /// Set a prefix used to rewrite bug reference links.
    ///
    /// For example, with a prefix of `https://example.org/bugs/`, a matched
    /// bug reference `#12345` will keep `as_str()` as `#12345` and return
    /// `https://example.org/bugs/12345` from `href()`.
    pub fn bug_reference_prefix(&mut self, prefix: &str) -> &mut LinkFinder {
        self.bug_reference_prefix = Some(prefix.to_owned());
        self
    }

    /// Restrict the kinds of links that should be found to the specified ones.
    pub fn kinds(&mut self, kinds: &[LinkKind]) -> &mut LinkFinder {
        self.bug_reference = false;
        self.email = false;
        self.url = false;
        for kind in kinds {
            match *kind {
                LinkKind::BugReference => self.bug_reference = true,
                LinkKind::Email => self.email = true,
                LinkKind::Url => self.url = true,
            }
        }
        self
    }

    /// Find links in the specified input text.
    ///
    /// Returns an `Iterator` which only scans when `next` is called (lazy).
    pub fn links<'t>(&self, text: &'t str) -> Links<'t> {
        Links::new(
            text,
            self.bug_reference,
            self.bug_reference_prefix.clone(),
            self.url,
            self.url_must_have_scheme,
            self.email,
            self.email_domain_must_have_dot,
            self.url_can_be_iri,
        )
    }

    /// Iterate over spans in the specified input text.
    ///
    /// A span represents a substring of the input text,
    /// which can either be a link, or plain text.
    ///
    /// Returns an `Iterator` which only scans when `next` is called (lazy).
    ///
    /// The spans that are returned by the `Iterator` are consecutive,
    /// and when combined represent the input text in its entirety.
    pub fn spans<'t>(&self, text: &'t str) -> Spans<'t> {
        Spans {
            text,
            position: 0,
            links: self.links(text).peekable(),
        }
    }
}

impl Default for LinkFinder {
    fn default() -> Self {
        LinkFinder::new()
    }
}

impl<'t> Links<'t> {
    fn new(
        text: &'t str,
        bug_reference: bool,
        bug_reference_prefix: Option<String>,
        url: bool,
        url_must_have_scheme: bool,
        email: bool,
        email_domain_must_have_dot: bool,
        iri_parsing_enabled: bool,
    ) -> Links<'t> {
        let bug_reference_scanner = BugReferenceScanner;
        let url_scanner = UrlScanner {
            iri_parsing_enabled,
        };
        let domain_scanner = DomainScanner {
            iri_parsing_enabled,
        };
        let email_scanner = EmailScanner {
            domain_must_have_dot: email_domain_must_have_dot,
        };

        // With optional schemes URLs don't have unique `:`, then search for `.` as well.
        let mut triggers = Vec::new();
        if url {
            triggers.push(b':');
            if !url_must_have_scheme {
                triggers.push(b'.');
            }
        }
        if email {
            triggers.push(b'@');
        }
        if bug_reference {
            triggers.push(b'#');
        }
        let trigger_finder: Box<TriggerFinder> =
            Box::new(move |s| s.iter().position(|byte| triggers.contains(byte)));
        Links {
            text,
            rewind: 0,
            bug_reference_prefix,
            trigger_finder,
            bug_reference_scanner,
            email_scanner,
            url_scanner,
            domain_scanner,
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
            let (scanner, kind): (&dyn Scanner, LinkKind) = match trigger {
                b'#' => (&self.bug_reference_scanner, LinkKind::BugReference),
                b':' => (&self.url_scanner, LinkKind::Url),
                b'.' => (&self.domain_scanner, LinkKind::Url),
                b'@' => (&self.email_scanner, LinkKind::Email),
                _ => unreachable!(),
            };
            if let Some(range) = scanner.scan(slice, find_from + i) {
                let start = self.rewind + range.start;
                let end = self.rewind + range.end;
                self.rewind = end;
                let href = if kind == LinkKind::BugReference {
                    self.bug_reference_prefix
                        .as_ref()
                        .map(|prefix| format!("{}{}", prefix, &self.text[start + 1..end]))
                } else {
                    None
                };
                let link = Link {
                    text: self.text,
                    start,
                    end,
                    href,
                    kind,
                };
                return Some(link);
            } else {
                // The scanner didn't find anything. But there could be more
                // trigger characters later, so continue the search.
                find_from += i + 1;
            }
        }

        None
    }
}

impl<'t> fmt::Debug for Links<'t> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Links").field("text", &self.text).finish()
    }
}

impl<'t> Iterator for Spans<'t> {
    type Item = Span<'t>;

    fn next(&mut self) -> Option<Span<'t>> {
        match self.links.peek() {
            Some(link) => {
                if self.position < link.start {
                    let span = Span {
                        text: self.text,
                        start: self.position,
                        end: link.start,
                        kind: None,
                    };
                    self.position = link.start;
                    return Some(span);
                }
            }
            None => {
                if self.position < self.text.len() {
                    let span = Span {
                        text: self.text,
                        start: self.position,
                        end: self.text.len(),
                        kind: None,
                    };
                    self.position = self.text.len();
                    return Some(span);
                }
            }
        };
        self.links.next().map(|link| {
            self.position = link.end;
            Span {
                text: self.text,
                start: link.start,
                end: link.end,
                kind: Some(link.kind),
            }
        })
    }
}

impl<'t> fmt::Debug for Spans<'t> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Spans").field("text", &self.text).finish()
    }
}
