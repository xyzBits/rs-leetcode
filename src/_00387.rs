use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn first_uniq_char_(s: String) -> i32 {
        let mut frequency = HashMap::new();
        for ch in s.chars() {
            let count = frequency.entry(ch).or_insert(0);
            *count += 1;
        }

        for (i, ch) in s.char_indices() {
            if let Some(&count) = frequency.get(&ch) {
                if count == 1 {
                    return i as i32;
                }
            }
        }

        -1
    }

    pub fn first_uniq_char(s: String) -> i32 {
        let mut position = HashMap::new();
        let n = s.len();
        for ch in s.chars() {
            if let None = position.get(&ch) {
                position.insert(ch, -1);
            } else {
                position.insert(ch, 1);
            }
        }

        let mut first = n as i32;
        for (key, value) in position {
            if value != -1 && value < first {
                first = value;
            }
        }

        if first == n as i32 {
            first = -1;
        }

        first
    }
}
