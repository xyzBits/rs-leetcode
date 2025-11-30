use crate::Solution;

impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let mut high_bit = 0;
        for i in 1..=30 {
            if num >= 1 << i {
                high_bit = i;
            } else {
                break;
            }
        }

        let mask = if high_bit == 30 {
            0x7fffffff
        } else {
            (1 << (high_bit + 1)) - 1
        };

        num ^ mask
    }
}
