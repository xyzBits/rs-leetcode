use crate::Solution;

impl Solution {
    pub fn palindrome_partition(s: String, k: i32) -> i32 {
        let n = s.len();
        let mut cost = vec![vec![0; n]; n];

        for span in 2..=n {
            for i in 0..=n - span {
                let j = i + span - 1;
                cost[i][j] = cost[i + 1][j - 1]
                    + if s.as_bytes()[i] == s.as_bytes()[j] {
                        0
                    } else {
                        1
                    };
            }
        }

        let mut f = vec![vec![i32::MAX; k as usize + 1]; n + 1];
        f[0][0] = 0;

        for i in 1..=n {
            for j in 1..=i.min(k as usize) {
                if j == 1 {
                    f[i][j] = cost[0][i - 1];
                } else {
                    for i0 in j - 1..i {
                        f[i][j] = f[i][j].min(f[i0][j - 1] + cost[i0][i - 1]);
                    }
                }
            }
        }

        f[n][k as usize]
    }
}
