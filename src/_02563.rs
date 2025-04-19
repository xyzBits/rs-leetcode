use crate::Solution;

impl Solution {
    pub fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        nums.sort();

        let mut ans = 0_i64;

        let mut left = nums.len();
        let mut right = nums.len();

        for j in 0..nums.len() {
            while right > 0 && nums[right - 1] > upper - nums[j] {
                right -= 1;
            }

            while left > 0 && nums[left - 1] >= lower - nums[j] {
                left -= 1;
            }

            ans += (right.min(j) as i64) - (left.min(j) as i64);
        }

        ans
    }
}
