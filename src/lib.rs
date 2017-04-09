//! Linkify finds links such as URLs and email addresses in plain text.
//! It's smart about where a link ends, such as with trailing punctuation.
//!
//! Your reaction might be: "Do I need a library for this? Why not a regex?".
//! Let's look at a few cases:
//!
//! * In `http://example.com/.` the link should not include the trailing dot
//! * `http://example.com/,` should not include the trailing comma
//! * `(http://example.com/)` should not include the parens
//!
//! Seems simple enough. But then we also have these cases:
//!
//! * `https://en.wikipedia.org/wiki/Link_(The_Legend_of_Zelda)` should include the trailing paren
//! * `http://example.com/ä` should also work for Unicode (including Emoji)
//! * `<http://example.com/>` should not include angle brackets
//!
//! This library behaves as you'd expect in the above cases and many more.
//! It uses a simple scan with linear runtime.
//!
//! In addition to URLs, it can also find emails.
//!
//! ### Conformance
//!
//! This crates makes an effort to respect the various standards, namely:
//!
//! * [RFC 3986] and [RFC 3987] for URLs
//! * [RFC 5321] and [RFC 6531] for emails (except IP addresses and quoting)
//!
//! At the same time, it does not guarantee that the returned links are valid.
//! If in doubt, it rather returns a link than skipping it.
//!
//! If you need to validate URLs, e.g. for checking TLDs, use another library on
//! the returned links.
//!
//! ### Usage
//!
//! Basic usage:
//!
//! ```
//! use linkify::LinkFinder;
//!
//! let input = "Have you seen http://example.com?";
//! let finder = LinkFinder::new();
//! let links: Vec<_> = finder.links(input).collect();
//!
//! assert_eq!(1, links.len());
//! let link = &links[0];
//!
//! assert_eq!("http://example.com", link.as_str());
//! assert_eq!(14, link.start());
//! assert_eq!(32, link.end());
//! ```
//!
//! Restrict the kinds of links:
//!
//! ```
//! use linkify::{LinkFinder, LinkKind};
//!
//! let input = "http://example.com and foo@example.com";
//! let mut finder = LinkFinder::new();
//! finder.kinds(&[LinkKind::Email]);
//! let links: Vec<_> = finder.links(input).collect();
//!
//! assert_eq!(1, links.len());
//! let link = &links[0];
//! assert_eq!("foo@example.com", link.as_str());
//! assert_eq!(&LinkKind::Email, link.kind());
//! ```
//!
//! [RFC 3986]: https://tools.ietf.org/search/rfc3986
//! [RFC 3987]: https://tools.ietf.org/search/rfc3987
//! [RFC 5321]: https://tools.ietf.org/search/rfc5321
//! [RFC 6531]: https://tools.ietf.org/search/rfc6531

extern crate memchr;

mod email;
mod finder;
mod url;
mod scanner;

pub use finder::Link;
pub use finder::LinkFinder;
pub use finder::LinkKind;
pub use finder::Links;
