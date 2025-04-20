use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        let mut count = HashMap::new();

        for &y in &answers {
            *count.entry(y).or_insert(0) += 1;
        }

        let mut ans = 0;

        for (&key, &value) in &count {
            ans += ((key + value) / (key + 1)) * (key + 1);
        }

        ans
    }
}
