use crate::Solution;

impl Solution {
    pub fn maximum_or(nums: Vec<i32>, k: i32) -> i64 {
        let (mut or_sum, mut multi_bits) = (0_i64, 0_i64);

        for &x in nums.iter() {
            multi_bits |= x as i64 & or_sum;
            or_sum |= x as i64;
        }

        let mut ans = 0;

        for &x in &nums {
            let val = x as i64;
            ans = ans.max((or_sum ^ val) | ((val << (k as i64)) | multi_bits));
        }

        ans
    }
}
