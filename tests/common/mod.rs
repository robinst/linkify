use linkify::LinkFinder;

pub fn assert_linked_with(finder: &LinkFinder, input: &str, expected: &str) {
    let actual = show_links(input, finder);
    assert_eq!(actual, expected);
}

pub fn show_links(input: &str, finder: &LinkFinder) -> String {
    let mut result = String::new();

    let mut i = 0;
    for link in finder.links(input) {
        result.push_str(&input[i..link.start()]);
        i = link.end();
        result.push('|');
        result.push_str(link.as_str());
        result.push('|');
    }
    result.push_str(&input[i..]);
    result
}

