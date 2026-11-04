pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let n = size as usize;
    let mut matrix = vec![vec![0; n]; n];

    if n == 0 {
        return matrix;
    }

    let (mut top, mut bottom) = (0, n - 1);
    let (mut left, mut right) = (0, n - 1);
    let mut value = 1;

    while top <= bottom && left <= right {
        // Move right
        for j in left..=right {
            matrix[top][j] = value;
            value += 1;
        }
        top += 1;

        // Move down
        for i in top..=bottom {
            matrix[i][right] = value;
            value += 1;
        }
        if left == right {
            break;
        }
        right -= 1;

        // Move left
        for j in (left..=right).rev() {
            matrix[bottom][j] = value;
            value += 1;
        }
        if top == bottom {
            break;
        }
        bottom -= 1;

        // Move up
        for i in (top..=bottom).rev() {
            matrix[i][left] = value;
            value += 1;
        }
        left += 1;
    }

    matrix
}
