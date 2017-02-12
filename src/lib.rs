//! Find links such as URLs and email addresses in plain text.
//! It's smart about where a link ends, such as with surrounding punctuation.
//!
//! Your first reaction might be: "That's easy, I can use a regex for that".
//! So you write one that kind of works with a few cases, such as:
//!
//! * `http://example.org/.` should not include the trailing dot
//! * `http://example.org/,` should not include the trailing comma
//! * `(http://example.org)` should not include the parens
//!
//! But then does it also work in these cases:
//!
//! * `https://en.wikipedia.org/wiki/Link_(The_Legend_of_Zelda)` should include the trailing paren
//! * `http://example.org/ä` should also work for Unicode of course
//! * `<http://example.org/>` should not include angle brackets
//!
//! This crate behaves as you'd expect in the above cases.
//!
//! ### Usage
//!
//! ```
//! use linkify::LinkFinder;
//!
//! let input = "Have you seen http://example.org?";
//! let finder = LinkFinder::new();
//! let links: Vec<_> = finder.links(input).collect();
//!
//! assert_eq!(1, links.len());
//! let link = &links[0];
//!
//! assert_eq!("http://example.org", link.as_str());
//! assert_eq!(14, link.start());
//! assert_eq!(32, link.end());
//! ```

extern crate memchr;

mod email;
mod finder;
mod url;
mod scanner;

pub use finder::Link;
pub use finder::LinkFinder;
pub use finder::LinkKind;
pub use finder::Links;
