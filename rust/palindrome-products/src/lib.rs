use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    val: u64,
    min: u64,
    max: u64,
}

impl Palindrome {
    pub fn value(&self) -> u64 {
        self.val
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        let mut factors = HashSet::new();

        for i in self.min..=self.max {
            if self.val.is_multiple_of(i) {
                let j = self.val / i;
                if i <= j && j <= self.max {
                    factors.insert((i, j));
                }
            }
        }

        factors
    }
}

fn is_palindrome(n: u64) -> bool {
    let mut original = n;
    let mut reversed = 0;

    while original > 0 {
        reversed = reversed * 10 + original % 10;
        original /= 10;
    }

    n == reversed
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    if min > max {
        return None;
    }

    let (mut min_palindrome, mut max_palindrome) = (None, None);

    for i in min..=max {
        for j in min..=i {
            let product = i * j;

            // Products increases as j increases.
            if min_palindrome.is_some_and(|p| product >= p) {
                break;
            }

            if is_palindrome(product) {
                min_palindrome = Some(product);
            }
        }

        // No remaining pair can beat the current minimum.
        if min_palindrome.is_some_and(|p| i * min > p) {
            break;
        }
    }

    for i in (min..=max).rev() {
        for j in (i..=max).rev() {
            let product = i * j;

            // Products decrease as j decreases.
            if max_palindrome.is_some_and(|p| product <= p) {
                break;
            }

            if is_palindrome(product) {
                max_palindrome = Some(product);
            }
        }

        // No remaining pair can beat the current maximum.
        if max_palindrome.is_some_and(|p| i * max < p) {
            break;
        }
    }

    match (min_palindrome, max_palindrome) {
        (Some(min_val), Some(max_val)) => Some((
            Palindrome {
                val: min_val,
                min,
                max,
            },
            Palindrome {
                val: max_val,
                min,
                max,
            },
        )),
        _ => None,
    }
}
