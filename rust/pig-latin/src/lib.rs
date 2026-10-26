pub fn translate(input: &str) -> String {
    input
        .split_whitespace()
        .map(translate_word)
        .collect::<Vec<_>>()
        .join(" ")
}

fn translate_word(word: &str) -> String {
    if word.starts_with(['a', 'e', 'i', 'o', 'u'])
        || word.starts_with("xr")
        || word.starts_with("yt")
    {
        return format!("{word}ay");
    }

    let split_idx = word
        .char_indices()
        .skip(1)
        .find_map(|(i, c)| {
            let prev = word[..i].chars().next_back().unwrap();

            if prev == 'q' && c == 'u' {
                None
            } else if matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'y') {
                Some(i)
            } else {
                None
            }
        })
        .unwrap_or(word.len());

    format!("{}{}ay", &word[split_idx..], &word[..split_idx])
}
