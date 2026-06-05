use criterion::{criterion_group, criterion_main, Criterion};

use linkify::LinkFinder;

fn no_links(c: &mut Criterion) {
    c.bench_function("no_links", |b| {
        let link_finder = LinkFinder::new();
        b.iter(|| {
            let links = link_finder.links(
                "This is a text with no links in it. But: it has a colon.\
                 Lorem ipsum dolor sit amet, consectetur adipiscing elit.\
                 Curabitur luctus tincidunt diam.",
            );
            assert_eq!(links.count(), 0);
        });
    });
}

fn some_links(c: &mut Criterion) {
    c.bench_function("some_links", |b| {
        let link_finder = LinkFinder::new();
        b.iter(|| {
            let links = link_finder.links(
                "This is a text with links in it. Here's one: http://www.example.com/. \
                 How about another one? (Maybe like this http://example.com/foo_(bar)) \
                 a+b://example.com/foo+bar+baz",
            );
            assert_eq!(links.count(), 3);
        });
    });
}

fn heaps_of_links(c: &mut Criterion) {
    c.bench_function("heaps_of_links", |b| {
        let link_finder = LinkFinder::new();
        b.iter(|| {
            let links = link_finder.links(
                "http://www.example.com/a, http://www.example.com/b, http://www.example.com/c \
                 http://www.example.com/a: http://www.example.com/b: http://www.example.com/c \
                 http://www.example.com/a http://www.example.com/b http://www.example.com/c \
                 http://www.example.com/a< http://www.example.com/b< http://www.example.com/c<",
            );
            assert_eq!(links.count(), 12);
        });
    });
}

fn some_links_without_scheme(c: &mut Criterion) {
    c.bench_function("some_links_without_scheme", |b| {
        let mut link_finder = LinkFinder::new();
        link_finder.url_must_have_scheme(false);
        b.iter(|| {
            let links = link_finder.links(
                "This is a text with links in it. Here's one: http://www.example.com/. \
             How about another one? (Maybe like this http://example.com/foo_(bar)). \
             example.com/one/two/three/four",
            );
            assert_eq!(links.count(), 3);
        });
    });
}

fn ipv6_links(c: &mut Criterion) {
    let finder = LinkFinder::new();

    // performance of a standard, valid ipv6 link
    c.bench_function("ipv6_standard", |b| {
        b.iter(|| {
            let links = finder.links("http://[2001:db8::1]:8080/path");
            assert_eq!(links.count(), 1);
        })
    });

    c.bench_function("ipv6_unclosed_bracket_fail_fast", |b| {
        b.iter(|| {
            let links = finder.links(
                "http://[2001:db8::1_malformed_text_continues_on_and_on\
			notice how it just keeps going on but doesnt end because there hasnt been a closing bracket",
            );
            assert_eq!(links.count(), 0);
        })
    });
}

criterion_group!(
    benches,
    no_links,
    some_links,
    heaps_of_links,
    some_links_without_scheme,
    ipv6_links
);
criterion_main!(benches);
