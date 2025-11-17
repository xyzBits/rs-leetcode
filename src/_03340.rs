use crate::Solution;

impl Solution {
    pub fn is_balanced_(num: String) -> bool {
        let (mut diff, mut sign) = (0, 1);

        for c in num.chars() {
            let d = c.to_digit(10).unwrap() as i32;
            diff += d * sign;
            sign = -sign;
        }

        diff == 0
    }
}
