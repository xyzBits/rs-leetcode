use crate::Solution;

impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut start = 0;
        for i in 0..nums.len() {
            if i > 0 && nums[i] <= nums[i - 1] {
                start = i;
            }
            ans = ans.max((i - start) as i32 + 1);
        }

        ans
    }
}
