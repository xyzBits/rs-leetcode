use crate::Solution;

impl Solution {
    pub fn max_total_reward_(mut reward_values: Vec<i32>) -> i32 {
        reward_values.sort();

        let m = reward_values[reward_values.len() - 1];

        let mut dp = vec![0; 2 * m as usize];
        dp[0] = 1;

        for x in reward_values {
            for k in (x..=2 * x - 1).rev() {
                if dp[(k - x) as usize] == 1 {
                    dp[k as usize] = 1;
                }
            }
        }

        let mut ans = 0;

        for i in 0..dp.len() {
            if dp[i] == 1 {
                ans = i;
            }
        }

        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_max_total_reward() {
        assert_eq!(Solution::max_total_reward_(vec![1, 1, 3, 3]), 4);
        assert_eq!(Solution::max_total_reward_(vec![1, 6, 4, 3, 2]), 11);
    }
}
