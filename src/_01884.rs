use crate::Solution;

impl Solution {
    pub fn two_egg_drop(n: i32) -> i32 {
        let mut f = vec![i32::MAX / 2; n as usize + 1];
        f[0] = 0;

        for i in 1..=(n as usize) {
            for k in 1..=i {
                f[i] = std::cmp::min(f[i], std::cmp::max(k as i32 - 1, f[i - k]) + 1);
            }
        }

        f[n as usize]
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::two_egg_drop(2), 2);
        assert_eq!(Solution::two_egg_drop(100), 14);
    }
}
