use crate::Solution;

impl Solution {
    pub fn min_flips(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut ans = 0;

        for i in 0..m / 2 {
            for j in 0..n / 2 {
                let cnt = grid[i][j]
                    + grid[i][n - 1 - j]
                    + grid[m - 1 - i][j]
                    + grid[m - 1 - i][n - 1 - j];
                ans += cnt.min(4 - cnt);
            }
        }

        let mut diff = 0;
        let mut cnt = 0;

        if m % 2 == 1 {
            for j in 0..n / 2 {
                if grid[m / 2][j] ^ grid[m / 2][n - 1 - j] != 0 {
                    diff += 1;
                } else {
                    cnt += grid[m / 2][j] * 2;
                }
            }
        }

        if n % 2 == 1 {
            for i in 0..m / 2 {
                if grid[i][n / 2] ^ grid[m - 1 - i][n / 2] != 0 {
                    diff += 1;
                } else {
                    cnt += grid[i][n / 2] * 2;
                }
            }
        }

        if m % 2 == 1 && n % 2 == 1 {
            ans += grid[m / 2][n / 2];
        }

        if diff > 0 {
            ans += diff;
        } else {
            ans += cnt % 4
        }

        ans
    }
}
