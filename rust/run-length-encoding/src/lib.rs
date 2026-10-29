pub fn encode(source: &str) -> String {
    let mut encoded = String::new();
    let mut chars = source.chars().peekable();

    while let Some(c) = chars.next() {
        let mut count = 1;

        while chars.peek() == Some(&c) {
            chars.next();
            count += 1;
        }

        if count > 1 {
            encoded.push_str(&count.to_string());
        }

        encoded.push(c);
    }

    encoded
}

pub fn decode(source: &str) -> String {
    let mut decoded = String::new();
    let mut count = 0usize;

    for c in source.chars() {
        if let Some(n) = c.to_digit(10) {
            count = count * 10 + n as usize;
        } else {
            let repetitions = if count == 0 { 1 } else { count };
            decoded.extend(std::iter::repeat_n(c, repetitions));
            count = 0;
        }
    }

    decoded
}
