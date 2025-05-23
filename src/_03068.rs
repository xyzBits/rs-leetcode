use crate::Solution;

impl Solution {
    pub fn maximum_value_sum(nums: Vec<i32>, k: i32, edges: Vec<Vec<i32>>) -> i64 {
        let mut ans = 0;

        let mut diff = vec![0; nums.len()];
        for i in 0..nums.len() {
            ans += nums[i] as i64;
            diff[i] = (nums[i] ^ k) - nums[i];
        }

        diff.sort();

        let mut i = diff.len();
        while i >= 2 && diff[i - 1] + diff[i - 2] >= 0 {
            ans += diff[i - 1] as i64 + diff[i - 2] as i64;
            i -= 2;
        }

        ans
    }
}
