use crate::Solution;

impl Solution {
    pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        for i in 0..n {
            let num = nums[i];
            let x = (num - 1) as usize % n;
            if nums[x] <= n as i32 {
                nums[x] += n as i32;
            }
        }

        let mut result = vec![];

        for i in 0..n {
            if nums[i] <= n as i32 {
                result.push(i as i32 + 1);
            }
        }

        result
    }
}
