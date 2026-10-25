use std::{cmp::Ordering, collections::HashSet};

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        None
    } else {
        match (1..=num.isqrt())
            .filter(|&i| num.is_multiple_of(i))
            .fold(HashSet::new(), |mut acc, i| {
                acc.extend([i, num / i]);
                acc
            })
            .iter()
            .filter(|&&i| i < num)
            .sum::<u64>()
            .cmp(&num)
        {
            Ordering::Less => Some(Classification::Deficient),
            Ordering::Equal => Some(Classification::Perfect),
            Ordering::Greater => Some(Classification::Abundant),
        }
    }
}
