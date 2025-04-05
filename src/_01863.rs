use crate::Solution;

impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut ans = 0;

        for num in nums {
            ans |= num;
        }

        ans << (n - 1)
    }
}
