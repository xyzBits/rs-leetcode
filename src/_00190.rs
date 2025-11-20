use crate::Solution;

impl Solution {
    pub fn reverse_bits(mut n: i32) -> i32 {
        // n.reverse_bits()

        let mut rev = 0;

        let mut i = 0;

        while i < 32 && n != 0 {
            rev |= (n & 1) << (31 - i);
            n >>= 1;
            i += 1;
        }

        rev
    }
}
