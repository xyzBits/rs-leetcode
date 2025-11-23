use crate::Solution;

impl Solution {
    pub fn count_bits(num: i32) -> Vec<i32> {
        let mut bits = vec![0; (num + 1) as usize];

        for i in 0..=num {
            bits[i as usize] = i.count_ones() as i32;
        }

        bits
    }
}
