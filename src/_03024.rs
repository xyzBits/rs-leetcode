use crate::Solution;

impl Solution {
    pub fn triangle_type(mut nums: Vec<i32>) -> String {
        nums.sort();
        if nums[0] + nums[1] <= nums[2] {
            "none".to_string()
        } else if nums[0] == nums[2] {
            "equilateral".to_string()
        } else if nums[0] == nums[1] || nums[1] == nums[2] {
            "isosceles".to_string()
        } else {
            "scalene".to_string()
        }
    }
}
