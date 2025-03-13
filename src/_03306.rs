use crate::Solution;
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn count(word: &str, m: i32) -> i64 {
        let mut vowels = HashSet::new();
        vowels.insert('a');
        vowels.insert('e');
        vowels.insert('i');
        vowels.insert('o');
        vowels.insert('u');

        let (n, mut consonants) = (word.len(), 0);
        let mut occur = HashMap::new();
        let mut j = 0;

        let word_chars = word.chars().collect::<Vec<char>>();
        let mut res = 0;

        for i in 0..n {
            while j < n && (consonants < m || occur.len() < 5) {
                let ch = word_chars[j];
                if vowels.contains(&ch) {
                    *occur.entry(ch).or_insert(0) += 1;
                } else {
                    consonants += 1;
                }
                j += 1;
            }

            if consonants >= m && occur.len() == 5 {
                res += n - j + 1;
            }

            let left = word_chars[i];
            if vowels.contains(&left) {
                *occur.entry(left).or_insert(0) -= 1;
                if let Some(0) = occur.get(&left) {
                    occur.remove(&left);
                }
            } else {
                consonants -= 1;
            }
        }

        res as i64
    }

    pub fn count_of_substrings(word: String, k: i32) -> i64 {
        Self::count(&word, k) - Self::count(&word, k + 1)
    }
}
