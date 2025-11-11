use crate::Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let len = digits.len();
        for i in (0..len).rev() {
            digits[i] = (digits[i] + 1) % 10;
            if digits[i] != 0 {
                return digits;
            }
        }

        digits = vec![0; len + 1];
        digits[0] = 1;
        digits
    }
}
