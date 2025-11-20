use crate::Solution;

impl Solution {
    pub fn hamming_weight(mut n: i32) -> i32 {
        let mut ret = 0;

        while n != 0 {
            n &= n - 1;
            ret += 1;
        }

        ret
    }
}
