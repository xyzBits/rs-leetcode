use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn count_good(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let mut same = 0;
        let mut right = -1_i32;
        let mut cnt = HashMap::new();
        let mut ans = 0_i64;

        for left in 0..n {
            while same < k && ((right + 1) as usize) < n {
                right += 1;
                let index = nums[right as usize];
                same += *cnt.get(&index).unwrap_or(&0);

                *cnt.entry(index).or_insert(0) += 1;
            }

            if same >= k {
                ans += n as i64 - right as i64;
            }

            let index = nums[left];
            *cnt.entry(index).or_insert(0) -= 1;
            same -= *cnt.get(&index).unwrap_or(&0);
        }

        ans
    }
}
