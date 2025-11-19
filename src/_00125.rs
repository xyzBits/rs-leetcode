use crate::Solution;

impl Solution {
    pub fn is_palindrome_125(s: String) -> bool {
        let sgood = s
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect::<String>();

        let sgood_rev = sgood.chars().rev().collect::<String>();

        sgood == sgood_rev
    }
}
