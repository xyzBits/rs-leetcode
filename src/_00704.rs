use crate::Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return -1;
        }
        let (mut left, mut right) = (0_i32, (nums.len() - 1) as i32);
        while left <= right {
            let mid = (right - left) / 2 + left;
            let num = nums[mid as usize];
            if num == target {
                return mid;
            } else if num > target {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        -1
    }
}
