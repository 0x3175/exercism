use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};

fn calc_factors(input: &str) -> (Vec<char>, Vec<i64>) {
    let mut factors = HashMap::<char, i64>::new();
    let mut sign = -1;
    let mut pos = 0;

    for ch in input.chars().filter(|c| !c.is_whitespace()).rev() {
        match ch {
            '=' => {
                sign = 1;
                pos = 0;
            }
            '+' => pos = 0,
            c => {
                *factors.entry(c).or_default() += sign * 10_i64.pow(pos);
                pos += 1;
            }
        }
    }

    factors
        .into_iter()
        .sorted_by_key(|(_, f)| Reverse(f.abs()))
        .unzip()
}

fn dfs(
    idx: usize,
    sum: i64,
    letters: &[char],
    factors: &[i64],
    leading: &HashSet<char>,
    used: &mut [bool; 10],
    assign: &mut [u8],
) -> bool {
    if idx == letters.len() {
        return sum == 0;
    }

    let letter = letters[idx];
    let factor = factors[idx];

    for digit in 0..10 {
        if used[digit] {
            continue;
        }

        if digit == 0 && leading.contains(&letter) {
            continue;
        }

        used[digit] = true;
        assign[idx] = digit as u8;

        if dfs(
            idx + 1,
            sum + factor * digit as i64,
            letters,
            factors,
            leading,
            used,
            assign,
        ) {
            return true;
        }

        used[digit] = false;
    }

    false
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let leading = input
        .split(&['+', '='])
        .filter_map(|s| s.trim().chars().next())
        .collect::<HashSet<_>>();

    let (letters, factors) = calc_factors(input);

    if letters.len() > 10 {
        return None;
    }

    let mut used = [false; 10];
    let mut assign = vec![0u8; letters.len()];

    if dfs(0, 0, &letters, &factors, &leading, &mut used, &mut assign) {
        Some(
            letters
                .iter()
                .copied()
                .zip(assign)
                .collect::<HashMap<_, _>>(),
        )
    } else {
        None
    }
}
