use crate::Solution;

impl Solution {
    pub fn judge_square_sum_1(c: i32) -> bool {
        if c == 0 {
            return true;
        }
        for a in 0..=((c as f64).sqrt() as i32) {
            let b_squared = c - a * a;
            if b_squared > 0 {
                let b = (b_squared as f64).sqrt();
                if b.fract() == 0.0 {
                    return true;
                }
            }
        }

        false
    }

    pub fn judge_square_sum(c: i32) -> bool {
        let mut left = 0_i64;
        let mut right = (c as f64).sqrt() as i64;

        while left <= right {
            let sum = (left * left + right * right) as i64;
            if sum == c as i64 {
                return true;
            } else if sum > c as i64 {
                right -= 1;
            } else {
                left += 1;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_judge_square_sum() {
        assert_eq!(Solution::judge_square_sum(5), true);
        assert_eq!(Solution::judge_square_sum(3), false);
        assert_eq!(Solution::judge_square_sum(0), true);
    }
}
