use crate::Solution;

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut ret = 0_u8;
        for ch in s.chars() {
            ret ^= ch as u8;
        }

        for ch in t.chars() {
            ret ^= ch as u8;
        }

        ret as _
    }
}
