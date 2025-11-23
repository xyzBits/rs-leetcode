use crate::Solution;

impl Solution {
    pub fn missing_number(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        let n = nums.len();
        for i in 0..n {
            if nums[i] != i as i32 {
                return i as i32;
            }
        }

        n as _
    }
}
