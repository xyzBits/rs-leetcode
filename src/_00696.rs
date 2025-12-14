use crate::Solution;

impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();

        if n < 2 {
            return 0;
        }

        let mut ptr = 0;
        let mut last = 0;
        let mut ans = 0;

        while ptr < n {
            let current_char = chars[ptr];
            let mut count = 0;

            while ptr < n && chars[ptr] == current_char {
                ptr += 1;
                count += 1;
            }

            ans += count.min(last);
            last = count;
        }

        ans
    }
}
