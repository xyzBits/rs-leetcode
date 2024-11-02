use crate::Solution;

impl Solution {
    pub fn min_changes(mut n: i32, mut k: i32) -> i32 {
        let mut ans = 0;

        while n > 0 || k > 0 {
            if (n & 1) == 0 && (k & 1) == 1 {
                return -1;
            }

            if (n & 1) == 1 && (k & 1) == 0 {
                ans += 1;
            }

            n >>= 1;
            k >>= 1;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::min_changes(13, 4), 2);
        assert_eq!(Solution::min_changes(21, 21), 0);
        assert_eq!(Solution::min_changes(14, 13), -1);
    }
}
