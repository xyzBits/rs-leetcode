use crate::Solution;

impl Solution {
    pub fn check_partitioning(s: String) -> bool {
        let n = s.len();
        let mut is_palindrome = vec![vec![false; n]; n];

        for length in 1..n {
            for start in 0..=n - length {
                let end = start + length - 1;
                if length == 1 {
                    is_palindrome[start][end] = true;
                } else if length == 2 {
                    is_palindrome[start][end] = s.as_bytes()[start] == s.as_bytes()[end];
                } else {
                    is_palindrome[start][end] = (s.as_bytes()[start] == s.as_bytes()[end])
                        && is_palindrome[start + 1][end - 1];
                }
            }
        }

        for start in 1..n - 1 {
            if !is_palindrome[0][start - 1] {
                continue;
            }
            for end in start..n - 1 {
                if is_palindrome[start][end] && is_palindrome[end + 1][n - 1] {
                    return true;
                }
            }
        }

        false
    }
}
