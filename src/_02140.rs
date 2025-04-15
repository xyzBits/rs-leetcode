use crate::Solution;

impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        let n = questions.len();

        let mut dp = vec![0; n + 1];

        for i in (0..n).rev() {
            dp[i] =
                dp[i + 1].max(questions[i][0] as i64 + dp[n.min(i + questions[i][1] as usize + 1)]);
        }

        dp[0]
    }
}
