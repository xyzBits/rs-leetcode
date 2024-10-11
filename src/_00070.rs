use crate::Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let (mut p, mut q, mut r) = (0, 0, 1);

        for _i in 1..=n {
            p = q;
            q = r;
            r = p + q;
        }

        r
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::climb_stairs(2), 2);
        assert_eq!(Solution::climb_stairs(3), 3);
    }
}
