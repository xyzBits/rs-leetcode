use crate::Solution;

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut result: Vec<Vec<i32>> = vec![];
        let row_index = row_index as usize;
        for i in 0..=row_index {
            let mut row = vec![];
            for j in 0..=i {
                if j == 0 || j == i {
                    row.push(1);
                } else {
                    let prev_row = &result[i - 1];
                    row.push(prev_row[j - 1] + prev_row[j]);
                }
            }
            result.push(row);
        }

        result[row_index].clone()
    }
}
