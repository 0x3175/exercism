use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let counts = nucleotide_counts(dna)?;
    counts.get(&nucleotide).copied().ok_or(nucleotide)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts = HashMap::from([('A', 0), ('C', 0), ('G', 0), ('T', 0)]);

    for c in dna.chars() {
        if let Some(count) = counts.get_mut(&c) {
            *count += 1;
        } else {
            return Err(c);
        }
    }

    Ok(counts)
}
