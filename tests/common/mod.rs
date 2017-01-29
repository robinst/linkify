use linkify::LinkFinder;

pub fn assert_linked_with(finder: &LinkFinder, input: &str, expected: &str) {
    let actual = show_links(input, finder);
    assert_eq!(actual, expected);
}

pub fn show_links(input: &str, finder: &LinkFinder) -> String {
    let mut result = String::new();

    let mut i = 0;
    for link in finder.links(input) {
        let range = link.range;
        result.push_str(&input[i..range.start]);
        i = range.end;
        result.push('|');
        result.push_str(&input[range]);
        result.push('|');
    }
    result.push_str(&input[i..]);
    result
}

