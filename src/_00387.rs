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
        let n = s.len() as i32;

        for (i, ch) in s.char_indices() {
            let i_i32 = i as i32;
            match position.entry(ch) {
                std::collections::hash_map::Entry::Occupied(mut entry) => {
                    entry.insert(-1);
                }
                std::collections::hash_map::Entry::Vacant(entry) => {
                    entry.insert(i_i32);
                }
            }
        }

        let mut first = n;
        for pos in position.values() {
            let pos = *pos;
            if pos != -1 && pos < first {
                first = pos;
            }
        }

        if first == n {
            first = -1;
        }

        first
    }
}
