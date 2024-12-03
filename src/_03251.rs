use crate::Solution;

impl Solution {
    pub fn count_of_pairs(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut m = 0;

        let mod_num = 1000000007;

        // for &x in &nums {
        //     m = m.max(x);
        // }
        m = *nums.iter().max().unwrap();

        let mut dp = vec![vec![0; (m + 1) as usize]; n];

        for a in 0..=nums[0] as usize {
            dp[0][a] = 1;
        }

        for i in 1..n {
            let d = 0.max(nums[i] - nums[i - 1]) as usize;
            for j in d..=nums[i] as usize {
                if j == 0 {
                    dp[i][j] = dp[i - 1][j - d];
                } else {
                    dp[i][j] = (dp[i][j - 1] + dp[i - 1][j - d]) % mod_num;
                }
            }
        }

        let mut ans = 0;

        for &num in &dp[n - 1] {
            ans = (ans + num) % mod_num;
        }

        ans
    }
}
