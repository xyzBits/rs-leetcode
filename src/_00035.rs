use crate::Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();

        let (mut left, mut right, mut ans) = (0, n - 1, n);
        while left <= right {
            let mid = left + (right - left) / 2;
            if nums[mid] == target {
                return mid as _;
            } else if nums[mid] < target {
                left = mid + 1;
            } else {
                if mid == 0 {
                    return mid as _;
                }
                right = mid - 1;
            }
        }

        left as _
    }
}
