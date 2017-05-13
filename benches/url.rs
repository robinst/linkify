#![feature(test)]
extern crate linkify;

extern crate test;

use linkify::LinkFinder;
use test::Bencher;

#[bench]
fn no_links(b: &mut Bencher) {
    let link_finder = LinkFinder::new();
    b.iter(|| {
               let links = link_finder
                   .links("This is a text with no links in it. But: it has a colon.\
             Lorem ipsum dolor sit amet, consectetur adipiscing elit.\
             Curabitur luctus tincidunt diam.");
               assert_eq!(links.count(), 0);
           });
}

#[bench]
fn some_links(b: &mut Bencher) {
    let link_finder = LinkFinder::new();
    b.iter(|| {
               let links = link_finder
                   .links("This is a text with links in it. Here's one: http://www.example.com/. \
            How about another one? (Maybe like this http://example.com/foo_(bar)) \
            a+b://example.com/foo+bar+baz");
               assert_eq!(links.count(), 3);
           });
}

#[bench]
fn heaps_of_links(b: &mut Bencher) {
    let link_finder = LinkFinder::new();
    b.iter(|| {
        let links = link_finder.links(
            "http://www.example.com/a, http://www.example.com/b, http://www.example.com/c \
             http://www.example.com/a: http://www.example.com/b: http://www.example.com/c \
             http://www.example.com/a http://www.example.com/b http://www.example.com/c \
             http://www.example.com/a< http://www.example.com/b< http://www.example.com/c<");
        assert_eq!(links.count(), 12);
    });
}
