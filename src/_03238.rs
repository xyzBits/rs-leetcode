use crate::Solution;

impl Solution {
    pub fn winning_player_count(n: i32, pick: Vec<Vec<i32>>) -> i32 {
        let mut cnt = vec![vec![0; 11]; n as usize];

        for p in &pick {
            cnt[p[0] as usize][p[1] as usize] += 1;
        }

        let mut ans = 0;

        for i in 0..n as usize {
            for j in 0..=10 {
                if cnt[i][j] > i {
                    ans += 1;
                    break;
                }
            }
        }

        ans
    }
}
