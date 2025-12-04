use crate::Solution;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n < 2 {
            return n;
        }

        let (mut p, mut q, mut r) = (0, 0, 1);
        for _ in 2..=n {
            p = q;
            q = r;
            r = p + q;
        }

        r
    }
}
