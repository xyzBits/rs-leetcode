use crate::Solution;
use std::collections::HashSet;

impl Solution {
    pub fn count_of_substrings(word: String, k: i32) -> i32 {
        let mut vowels = HashSet::new();
        vowels.insert('a');
        vowels.insert('e');
        vowels.insert('i');
        vowels.insert('o');
        vowels.insert('u');

        let n = word.len();

        let mut res = 0;
        for i in 0..n {
            let mut occur = HashSet::new();
            let mut consonants = 0;

            for j in i..n {
                if vowels.contains(&word[j..j + 1].chars().next().unwrap()) {
                    occur.insert(word[j..j + 1].chars().next().unwrap());
                } else {
                    consonants += 1;
                }

                if occur.len() == 5 && consonants == k {
                    res += 1;
                }
            }
        }

        res
    }
}
