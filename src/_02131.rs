use crate::Solution;
use std::cmp::min;
use std::collections::HashMap;

impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut freq = HashMap::new();
        for word in &words {
            *freq.entry(word.clone()).or_insert(0) += 1;
        }
        let mut res = 0;
        let mut mid = false;
        for (word, cnt) in &freq {
            let rev = format!(
                "{}{}",
                word.chars().nth(1).unwrap(),
                word.chars().nth(0).unwrap()
            );
            if *word == rev {
                if cnt % 2 == 1 {
                    mid = true;
                }
                res += 2 * (cnt / 2 * 2);
            } else if *word > rev {
                res += 4 * min(*cnt, *freq.get(&rev).unwrap_or(&0));
            }
        }
        if mid {
            res += 2;
        }
        res
    }
}
