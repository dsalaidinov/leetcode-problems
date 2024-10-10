Link: https://leetcode.com/problems/diagonal-traverse/

impl Solution {
    pub fn find_diagonal_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.is_empty() || matrix[0].is_empty() {
            return vec![];
        }

        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut result = Vec::with_capacity(rows * cols);
        let mut r = 0;
        let mut c = 0;
        let mut up = true;

        while result.len() < rows * cols {
            result.push(matrix[r][c]);

            if up {
                if c == cols - 1 {
                    r += 1;
                    up = false;
                } else if r == 0 {
                    c += 1;
                    up = false;
                } else {
                    r -= 1;
                    c += 1;
                }
            } else {
                if r == rows - 1 {
                    c += 1;
                    up = true;
                } else if c == 0 {
                    r += 1;
                    up = true;
                } else {
                    r += 1;
                    c -= 1;
                }
            }
        }
        result
    }
}