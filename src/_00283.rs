use crate::Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let (n, mut left, mut right) = (nums.len(), 0, 0);

        while right < n {
            if nums[right] != 0 {
                let temp = nums[left];
                nums[left] = nums[right];
                nums[right] = temp;
                left += 1;
            }

            right += 1;
        }
    }
}
