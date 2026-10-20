use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut seen = HashSet::new();

    candidate
        .chars()
        .filter(|c| *c != ' ' && *c != '-')
        .all(|c| seen.insert(c.to_ascii_lowercase()))
}
