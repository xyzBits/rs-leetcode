use crate::Solution;

impl Solution {
    pub fn count_subarrays_2303(nums: Vec<i32>, k: i64) -> i64 {
        let n = nums.len();
        let mut res = 0_i64;
        let mut total = 0_i64;

        let mut i = 0_usize;
        for j in 0..n {
            total += nums[j] as i64;

            while i <= j && total * (j - i + 1) as i64 >= k {
                total -= nums[i] as i64;
                i += 1;
            }
            res += (j - i + 1) as i64;
        }

        res
    }
}
