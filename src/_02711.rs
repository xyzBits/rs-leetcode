use crate::Solution;
use std::collections::HashSet;

impl Solution {
    pub fn difference_of_distinct_values(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();

        let mut res = vec![vec![0; n]; m];

        for i in 0..m {
            for j in 0..n {
                let mut s1 = HashSet::new();
                let mut x = i + 1;
                let mut y = j + 1;

                while x < m && y < n {
                    s1.insert(grid[x][y]);
                    x += 1;
                    y += 1;
                }

                let mut s2 = HashSet::new();
                let mut x = i as i32 - 1;
                let mut y = j as i32 - 1;
                while x >= 0 && y >= 0 {
                    s2.insert(grid[x as usize][y as usize]);
                    x -= 1;
                    y -= 1;
                }

                res[i][j] = s1.len().abs_diff(s2.len()) as i32;
            }
        }

        res
    }
}
