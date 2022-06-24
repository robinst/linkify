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
                // Can start or end a domain label, but not numeric.
                dot_allowed = true;
                hyphen_allowed = true;
                last_dot = maybe_last_dot;
                all_numeric = false;

                true
            }
            '0'..='9' => {
                // Same as above
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

pub(crate) fn find_authority_end(s: &str) -> Option<usize> {
    let mut port = false;
    let mut end = Some(0);

    for (i, c) in s.char_indices() {
        let can_be_last = match c {
            '.' => {
                // . at end of domain allowed, but only if we have a / or port and slash after
                if i != 0 {
                    break;
                }
                false
            }
            ':' => {
                if port {
                    break;
                }
                port = true;
                false
            }
            '0'..='9' => {
                if !port {
                    break;
                }
                true
            }
            _ => {
                break;
            }
        };

        if can_be_last {
            end = Some(i + c.len_utf8());
        }
    }

    end
}

pub(crate) fn find_authority(
    s: &str,
    mut userinfo_allowed: bool,
    require_host: bool,
) -> Option<usize> {
    let mut end = Some(0);

    let mut maybe_last_dot = None;
    let mut last_dot = None;
    let mut dot_allowed = false;
    let mut hyphen_allowed = false;
    let mut all_numeric = true;
    let mut maybe_host = true;
    let mut host_ended = false;

    for (i, c) in s.char_indices() {
        let can_be_last = match c {
            // ALPHA
            'a'..='z' | 'A'..='Z' | '\u{80}'..=char::MAX => {
                // Can start or end a domain label, but not numeric.
                dot_allowed = true;
                hyphen_allowed = true;
                last_dot = maybe_last_dot;
                all_numeric = false;

                // if host_ended {
                //     maybe_host = false;
                // }

                !require_host || !host_ended
            }
            // DIGIT
            '0'..='9' => {
                // Same as above
                dot_allowed = true;
                hyphen_allowed = true;
                last_dot = maybe_last_dot;

                // if host_ended {
                //     maybe_host = false;
                // }

                !require_host || !host_ended
            }
            // unreserved
            '-' => {
                // Hyphen can't be at start of a label, e.g. `-b` in `a.-b.com`
                if !hyphen_allowed {
                    maybe_host = false;
                }
                // Hyphen can't be at end of a label, e.g. `b-` in `a.b-.com`
                dot_allowed = false;

                !require_host
            }
            '.' => {
                if !dot_allowed {
                    // Label can't be empty, e.g. `.example.com` or `a..com`
                    maybe_host = false;
                }
                dot_allowed = false;
                hyphen_allowed = false;
                maybe_last_dot = Some(i);

                !require_host
            }
            '_' | '~' => {
                // Hostnames can't contain these
                maybe_host = false;
                // TODO: use host_ended or something, so we can distinguish between invalid host or host with trailing stuff?

                false
            }
            // sub-delims
            '!' | '$' | '&' | '\'' | '(' | ')' | '*' | '+' | ',' | ';' | '=' => {
                // TODO: What about something like https://a.com,https://b.com, should we try to support that?
                // Can't be in hostnames
                host_ended = true;

                false
            }
            ':' => {
                // Could be in userinfo, or we're getting a port now.
                if !userinfo_allowed {
                    // TODO: Just scan for port, then we're done.
                    //  but not for emails, hmmmm... Maybe do that outside?
                    // break;
                    // host_ended = true;
                }
                // Not sure
                false
            }
            '@' => {
                if !userinfo_allowed {
                    // We already had userinfo, can't have another `@` in a valid authority.
                    return None;
                }

                // Sike! Everything before this has been userinfo, so let's reset our
                // opinions about all the host bits.
                userinfo_allowed = false;

                maybe_last_dot = None;
                last_dot = None;
                dot_allowed = false;
                hyphen_allowed = false;
                all_numeric = true;
                maybe_host = true;
                host_ended = false;

                false
            }
            '/' => {
                if !require_host {
                    // For schemes where we allow anything, we want to stop at delimiter characters
                    // except if we get a slash closing the URL, which happened here.
                    end = Some(i);
                }
                break;
            }
            _ => {
                // Anything else, this might be the end of the authority (can be empty).
                // Now let the rest of the code handle checking whether the end of the URL is
                // valid.
                break;
            }
        };

        if can_be_last {
            end = Some(i + c.len_utf8());
        }
    }

    if require_host {
        if maybe_host {
            // TODO: more checking?

            if all_numeric && last_dot.is_none() {
                return None;
            }

            if !all_numeric {
                if let Some(last_dot) = last_dot {
                    if !valid_tld(&s[last_dot + 1..]) {
                        return None;
                    }
                }
            }

            return end;
        } else {
            return None;
        }
    } else {
        return end;
    }
}

fn valid_tld(tld: &str) -> bool {
    tld.chars()
        .take_while(|c| c.is_ascii_alphabetic())
        .take(2)
        .count()
        >= 2
}
