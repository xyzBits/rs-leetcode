use crate::Solution;

impl Solution {
    pub fn count_of_pairs_(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![vec![0; 51]; n];
        let mod_num = 1000000007;

        for v in 0..=nums[0] as usize {
            dp[0][v] = 1;
        }

        for i in 1..n {
            for v2 in 0..=nums[i] as usize {
                for v1 in 0..=v2 {
                    if nums[i - 1] - v1 as i32 >= nums[i] - v2 as i32 && nums[i] - v2 as i32 >= 0 {
                        dp[i][v2] = (dp[i][v2] + dp[i - 1][v1]) % mod_num;
                    }
                }
            }
        }

        let mut ans = 0;

        for &v in &dp[n - 1] {
            ans = (ans + v) % mod_num;
        }

        ans
    }
}
