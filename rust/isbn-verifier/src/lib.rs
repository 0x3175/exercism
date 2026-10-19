/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let cleaned: String = isbn.chars().filter(|&c| c != '-').collect();
    if cleaned.len() != 10 {
        return false;
    }

    let bytes = cleaned.as_bytes();
    let mut sum = 0;

    for (i, &b) in bytes.iter().enumerate() {
        let digit = match b {
            b'0'..=b'9' => (b - b'0') as i32,
            b'X' if i == 9 => 10, // 'X' is only valid as the last character
            _ => return false,
        };
        let weight = (10 - i) as i32;
        sum += digit * weight;
    }

    sum % 11 == 0
}
