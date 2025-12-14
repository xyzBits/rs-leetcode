use crate::Solution;

impl Solution {
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        nums.sort();

        let mut dp = vec![1; len];

        let mut max_size = 1;
        let mut max_val = dp[0];
        for i in 1..len {
            for j in 0..i {
                if nums[i] % nums[j] == 0 {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }

            if dp[i] > max_size {
                max_size = dp[i];
                max_val = nums[i];
            }
        }

        let mut res = Vec::new();
        if max_size == 1 {
            res.push(nums[0]);
            return res;
        }

        let mut i = (len - 1) as isize;
        while i >= 0 && max_size > 0 {
            if dp[i as usize] == max_size && max_val % nums[i as usize] == 0 {
                res.push(nums[i as usize]);
                max_val = nums[i as usize];
                max_size -= 1;
            }
            i -= 1;
        }

        res
    }
}
