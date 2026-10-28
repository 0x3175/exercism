use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        validate(dna, "ACGT").map(Dna)
    }

    pub fn into_rna(self) -> Rna {
        let mapping = HashMap::from([('G', 'C'), ('C', 'G'), ('T', 'A'), ('A', 'U')]);
        Rna(self.0.chars().map(|c| mapping[&c]).collect())
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        validate(rna, "ACGU").map(Rna)
    }
}

fn validate(s: &str, chars: &str) -> Result<String, usize> {
    match s.chars().position(|c| !chars.contains(c)) {
        Some(i) => Err(i),
        None => Ok(s.to_string()),
    }
}
