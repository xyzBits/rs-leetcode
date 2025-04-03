use crate::Solution;

impl Solution {
    fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut left_max = vec![0; n];
        let mut right_max = vec![0; n];

        for i in 1..n {
            left_max[i] = left_max[i - 1].max(nums[i - 1]);
            right_max[n - 1 - i] = right_max[n - i].max(nums[n - i]);
        }

        let mut res = 0;

        for j in 1..n - 1 {
            res = res.max((left_max[j] - nums[j]) as i64 * right_max[j] as i64);
        }

        res
    }
}
