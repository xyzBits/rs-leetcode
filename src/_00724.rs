use crate::Solution;

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let total: i32 = nums.iter().sum();
        let mut sum = 0;
        for (i, &num) in nums.iter().enumerate() {
            if 2 * sum + num == total {
                return i as i32;
            }
            sum += num;
        }

        -1
    }
}
