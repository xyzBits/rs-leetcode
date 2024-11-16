use crate::Solution;

impl Solution {
    pub fn min_flips_(grid: Vec<Vec<i32>>) -> i32 {
        let mut row_cnt = 0;
        let mut col_cnt = 0;

        let m = grid.len();
        let n = grid[0].len();

        for i in 0..m {
            let mut j1 = 0;
            let mut j2 = n - 1;
            while j1 < j2 {
                if grid[i][j1] ^ grid[i][j2] != 0 {
                    row_cnt += 1;
                }

                j1 += 1;
                j2 -= 1;
            }
        }

        for j in 0..n {
            let mut i1 = 0;
            let mut i2 = m - 1;
            while i1 < i2 {
                if grid[i1][j] ^ grid[i2][j] != 0 {
                    col_cnt += 1;
                }

                i1 += 1;
                i2 -= 1;
            }
        }

        row_cnt.min(col_cnt)
    }
}
