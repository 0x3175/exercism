const ONES: [&str; 20] = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

const TENS: [&str; 10] = [
    "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

const SCALES: [&str; 7] = [
    "",
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
];

fn encode_group(n: u64) -> String {
    match n {
        0 => String::new(),
        1..=19 => ONES[n as usize].to_string(),
        20..=99 => {
            let tens = TENS[(n / 10) as usize];
            let ones = n % 10;

            if ones == 0 {
                tens.to_string()
            } else {
                format!("{tens}-{}", ONES[ones as usize])
            }
        }
        100..=999 => {
            let hundreds = ONES[(n / 100) as usize];
            let remainder = encode_group(n % 100);

            if remainder.is_empty() {
                format!("{hundreds} hundred")
            } else {
                format!("{hundreds} hundred {remainder}")
            }
        }
        _ => unreachable!(),
    }
}

pub fn encode(mut n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }

    let mut parts = Vec::new();

    for scale in SCALES {
        let group = n % 1000;

        if group != 0 {
            let words = encode_group(group);

            parts.push(if scale.is_empty() {
                words
            } else {
                format!("{words} {scale}")
            });
        }

        n /= 1000;

        if n == 0 {
            break;
        }
    }

    parts.reverse();
    parts.join(" ")
}
