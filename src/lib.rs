//! TODO.
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
//! assert_eq!("http://example.org", &input[link.range.clone()]);
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
