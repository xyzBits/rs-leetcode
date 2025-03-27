use crate::Solution;

impl Solution {
    pub fn minimum_cost(s: String) -> i64 {
        let n = s.len();
        let mut res = 0;

        let bytes = s.as_bytes();

        for i in 1..n {
            if bytes[i] != bytes[i - 1] {
                res += i.min(n - i) as i64;
            }
        }

        res
    }
}
