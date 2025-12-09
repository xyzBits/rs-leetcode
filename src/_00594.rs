use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut cnt = HashMap::new();
        let mut res = 0;

        for num in nums {
            let value = cnt.entry(num).or_insert(0);
            *value += 1;
        }

        for &key in cnt.keys() {
            if cnt.contains_key(&(key + 1)) {
                res = res.max(cnt[&key] + cnt[&(key + 1)]);
            }
        }

        res
    }
}
