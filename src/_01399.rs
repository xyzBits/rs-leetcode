use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let mut map = HashMap::new();

        let mut max_value = 0;
        for i in 1..=n {
            let (mut key, mut i0) = (0, i);
            while i0 > 0 {
                key += i0 % 10;
                i0 /= 10;
            }
            *map.entry(key).or_insert(0) += 1;
            max_value = max_value.max(map[&key]);
        }

        let mut count = 0;

        for (&_key, &value) in &map {
            if value == max_value {
                count += 1;
            }
        }

        count
    }
}
