//! Domain name related scanning, used by both email and plain domain URL scanners.

pub(crate) fn find_domain_end(s: &str) -> (Option<usize>, Option<usize>) {
    let mut end = None;
    let mut maybe_last_dot = None;
    let mut last_dot = None;
    let mut dot_allowed = false;
    let mut hyphen_allowed = false;
    let mut all_numeric = true;

    for (i, c) in s.char_indices() {
        let can_be_last = match c {
            'a'..='z' | 'A'..='Z' | '\u{80}'..=char::MAX => {
                // Can start or end a domain label.
                dot_allowed = true;
                hyphen_allowed = true;
                last_dot = maybe_last_dot;
                all_numeric = false;

                true
            }
            '0'..='9' => {
                // Same as above, except we note if it's
                dot_allowed = true;
                hyphen_allowed = true;
                last_dot = maybe_last_dot;

                true
            }
            '-' => {
                // Hyphen can't be at start of a label, e.g. `-b` in `a.-b.com`
                if !hyphen_allowed {
                    return (None, None);
                }
                // Hyphen can't be at end of a label, e.g. `b-` in `a.b-.com`
                dot_allowed = false;
                false
            }
            '.' => {
                if !dot_allowed {
                    // Label can't be empty, e.g. `.example.com` or `a..com`
                    return (None, None);
                }
                dot_allowed = false;
                hyphen_allowed = false;
                maybe_last_dot = Some(i);
                false
            }
            _ => {
                break;
            }
        };

        if can_be_last {
            end = Some(i + c.len_utf8());
        }
    }

    if all_numeric && last_dot.is_none() {
        return (None, None);
    }

    if !all_numeric {
        if let Some(last_dot) = last_dot {
            if !valid_tld(&s[last_dot + 1..]) {
                return (None, None);
            }
        }
    }

    (end, last_dot)
}

fn valid_tld(tld: &str) -> bool {
    tld.chars()
        .take_while(|c| c.is_ascii_alphabetic())
        .take(2)
        .count()
        >= 2
}
