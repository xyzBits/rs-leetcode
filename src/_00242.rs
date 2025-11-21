use crate::Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut s_chars = s.chars().collect::<Vec<char>>();
        let mut t_chars = t.chars().collect::<Vec<char>>();

        s_chars.sort_unstable();
        t_chars.sort_unstable();

        s_chars == t_chars
    }
}
