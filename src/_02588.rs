use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn beautiful_subarrays(nums: Vec<i32>) -> i64 {
        let mut cnt = HashMap::new();
        let mut mask = 0;
        let mut ans = 0;

        cnt.insert(0, 1);

        for x in nums {
            mask ^= x;
            ans += cnt.get(&mask).unwrap_or(&0);
            cnt.insert(mask, cnt.get(&mask).unwrap_or(&0) + 1);
        }

        ans
    }
}
