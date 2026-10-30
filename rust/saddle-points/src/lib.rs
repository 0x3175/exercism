pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    if input.is_empty() || input[0].is_empty() {
        return vec![];
    }

    let mut points = Vec::new();

    for (i, row) in input.iter().enumerate() {
        let max = *row.iter().max().unwrap();

        for (j, &val) in row.iter().enumerate() {
            if val == max && input.iter().all(|r| r[j] >= max) {
                points.push((i, j));
            }
        }
    }

    points
}
