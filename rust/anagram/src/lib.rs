use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    fn sort_and_lower(s: &str) -> String {
        let mut chars: Vec<char> = s.to_lowercase().chars().collect();
        chars.sort_unstable();
        chars.into_iter().collect()
    }

    let sorted_word = sort_and_lower(word);
    possible_anagrams
        .iter()
        .filter(|&&candidate| {
            candidate.to_lowercase() != word.to_lowercase()
                && sort_and_lower(candidate) == sorted_word
        })
        .copied()
        .collect()
}
