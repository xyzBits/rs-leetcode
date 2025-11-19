use crate::Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let n = num_rows as usize;
        let mut result: Vec<Vec<i32>> = Vec::new();

        for i in 0..n {
            let mut row = Vec::new();
            for j in 0..=i {
                if j == 0 || j == i {
                    row.push(1);
                } else {
                    // let pre_row = &result[i - 1];
                    row.push(&result[i - 1][j - 1] + &result[i - 1][j]);
                }
            }

            result.push(row);
        }

        result
    }
}
