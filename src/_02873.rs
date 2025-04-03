use crate::Solution;

impl Solution {
    pub fn maximum_triplet_value_2873(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut res = 0;

        for i in 0..n {
            for j in i + 1..n {
                for k in j + 1..n {
                    res = res.max(((nums[i] - nums[j]) as i64 * nums[k] as i64))
                }
            }
        }

        res
    }
}
