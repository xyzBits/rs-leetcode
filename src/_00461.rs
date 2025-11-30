use crate::Solution;

impl Solution {
    pub fn hamming_distance_(x: i32, y: i32) -> i32 {
        (x ^ y).count_ones() as i32
    }

    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let mut s = x ^ y;
        let mut ret = 0;

        while s != 0 {
            s &= s - 1;
            ret += 1;
        }

        ret
    }
}
