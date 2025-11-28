use crate::Solution;

impl Solution {
    pub fn repeated_substring_pattern_(s: String) -> bool {
        let n = s.len();
        let mut i = 1;
        while i * 2 <= n {
            if n % i == 0 {
                let mut matched = true;
                for j in i..n {
                    if s.chars().nth(j).unwrap() != s.chars().nth(j - i).unwrap() {
                        matched = false;
                        break;
                    }
                }

                if matched {
                    return true;
                }
            }

            i += 1;
        }

        false
    }

    pub fn repeated_substring_pattern(s: String) -> bool {
        let n = s.len();

        let doubled_s = format!("{}{}", s, s);
        let search_window = &doubled_s[1..2 * n - 1];
        search_window.contains(&s)
    }
}
