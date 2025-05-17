use crate::Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let n = nums.len();
        let mut ptr = 0;
        for i in 0..n {
            if nums[i] == 0 {
                let temp = nums[i];
                nums[i] = nums[ptr];
                nums[ptr] = temp;
                ptr += 1;
            }
        }

        for i in ptr..n {
            if nums[i] == 1 {
                let temp = nums[i];
                nums[i] = nums[ptr];
                nums[ptr] = temp;
                ptr += 1;
            }
        }
    }
}
