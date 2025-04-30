use crate::Solution;

impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        let mut ans = 0;

        for num in nums {
            if num.to_string().len() % 2 == 0 {
                ans += 1;
            }
        }

        ans
    }
}
