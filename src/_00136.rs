use crate::Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut single = 0;
        nums.iter().for_each(|&x| {
            single = single ^ x;
        });

        single
    }
}
