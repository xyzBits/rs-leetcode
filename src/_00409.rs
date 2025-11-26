use crate::Solution;

impl Solution {
    pub fn longest_palindrome_409(s: String) -> i32 {
        let mut count = vec![0; 128];

        for ch in s.chars() {
            count[ch as usize] += 1;
        }

        let mut ans = 0;

        for v in count {
            ans += v / 2 * 2;
            if v % 2 == 1 && ans % 2 == 0 {
                ans += 1;
            }
        }

        ans
    }
}
