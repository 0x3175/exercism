const PLANT: u8 = 9;

const DIRS: [(i8, i8); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

pub fn annotate(garden: &[&str]) -> Vec<String> {
    if garden.len() == 0 {
        return vec![];
    }

    let w = garden[0].len();
    let h = garden.len();
    let mut counts = vec![vec![0; w]; h];

    for (y, &row) in garden.iter().enumerate() {
        for (x, ch) in row.bytes().enumerate() {
            if ch == b'*' {
                counts[y][x] += PLANT;
                for (dx, dy) in DIRS {
                    let nx = x as isize + dx as isize;
                    let ny = y as isize + dy as isize;
                    if nx >= 0 && nx < w as isize && ny >= 0 && ny < h as isize {
                        counts[ny as usize][nx as usize] += 1;
                    }
                }
            }
        }
    }

    counts
        .iter()
        .map(|row| {
            row.iter()
                .map(|&count| match count {
                    0 => ' ',
                    1..=8 => char::from_digit(count as u32, 10).unwrap(),
                    _ => '*',
                })
                .collect()
        })
        .collect()
}
