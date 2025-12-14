use crate::Solution;

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let n = nums.len();
        if n < 2 {
            return false;
        }

        let sum = nums.iter().sum::<i32>();
        let max_num = *nums.iter().max().unwrap();

        if sum % 2 != 0 {
            return false;
        }

        let target = sum / 2;
        if max_num > target {
            return false;
        }

        let mut dp = vec![false; (target + 1) as usize];
        dp[0] = true;

        for i in 0..n {
            let num = nums[i];
            for j in (num..=target).rev() {
                dp[j as usize] = dp[(j - num) as usize];
            }
        }

        dp[target as usize]
    }
}
