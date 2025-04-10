use crate::Solution;
use std::cmp::Ordering;

impl Solution {
    pub fn number_of_powerful_int(start: i64, finish: i64, limit: i32, s: String) -> i64 {
        let start_ = (start - 1).to_string();
        let finish_ = finish.to_string();

        Self::calculate(&finish_, &s, limit) - Self::calculate(&start_, &s, limit)
    }

    fn calculate(x: &str, s: &str, limit: i32) -> i64 {
        if x.len() < s.len() {
            return 0;
        }

        if x.len() == s.len() {
            return if x >= s { 1 } else { 0 };
        }

        let suffix = &x[x.len() - s.len()..];
        let mut count = 0_i64;
        let pre_len = x.len() - s.len();

        for i in 0..pre_len {
            let digit = x.chars().nth(i).unwrap().to_digit(10).unwrap() as i32;
            if limit < digit {
                count += (limit as i64 + 1).pow(pre_len as u32 - i as u32);
                return count;
            }
            count += (digit as i64) * (limit as i64 + 1).pow(pre_len as u32 - i as u32 - 1);
        }

        if suffix.cmp(&s) >= Ordering::Equal {
            count += 1
        }

        count
    }
}
