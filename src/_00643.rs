use crate::Solution;

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut sum = 0;
        let n = nums.len();
        sum += nums.iter().take(k as usize).sum::<i32>();

        let mut max_sum = sum;

        let k = k as usize;
        for i in k..n {
            sum = sum - nums[i - k] + nums[i];
            max_sum = max_sum.max(sum);
        }

        max_sum as f64 / k as f64
    }
}
