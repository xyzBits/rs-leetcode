use crate::Solution;

const DIRS: [[i32; 2]; 8] = [
    [-2, -1],
    [-2, 1],
    [2, -1],
    [2, 1],
    [-1, -2],
    [-1, 2],
    [1, -2],
    [1, 2],
];

impl Solution {
    pub fn knight_probability(n: i32, k: i32, row: i32, column: i32) -> f64 {
        let n = n as usize;
        let k = k as usize;
        let mut dp = vec![vec![vec![0_f64; n]; n]; k + 1];

        for step in 0..=k {
            for i in 0..n {
                for j in 0..n {
                    if step == 0 {
                        dp[step][i][j] = 1.0;
                    } else {
                        for dir in &DIRS {
                            let ni = i + dir[0] as usize;
                            let nj = j + dir[1] as usize;
                            if ni < n && nj < n {
                                dp[step][i][j] += dp[step - 1][ni][nj] / 8.0;
                            }
                        }
                    }
                }
            }
        }

        dp[k as usize][row as usize][column as usize]
    }
}
