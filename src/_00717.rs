use crate::Solution;

impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let n = bits.len();
        let mut i = n as i32 - 2;

        while i >= 0 && bits[i as usize] == 1 {
            i -= 1;
        }

        (n - i as usize) % 2 == 0
    }
}
