use crate::Solution;

impl Solution {
    pub fn minimized_string_length(s: String) -> i32 {
        let mut mask = 0_u32;

        for c in s.chars() {
            mask |= 1 << (c as u8 - b'a');
        }

        mask.count_ones() as i32
    }
}
