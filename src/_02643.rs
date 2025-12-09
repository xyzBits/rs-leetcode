use crate::Solution;

impl Solution {
    pub fn row_and_maximum_ones(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let mut max_ones = 0;
        let mut row_index = 0;

        for i in 0..mat.len() {
            let mut tot = 0;
            for j in 0..mat[i].len() {
                tot += mat[i][j];
            }

            if tot > max_ones {
                max_ones = tot;
                row_index = i;
            }
        }

        vec![row_index as i32, max_ones]
    }
}
