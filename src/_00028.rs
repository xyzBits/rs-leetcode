use crate::Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let (n, m) = (haystack.len(), needle.len());

        let mut i = 0;
        while i + m <= n {
            let mut flag = true;
            for j in 0..m {
                if haystack.chars().nth(i + j).unwrap() != needle.chars().nth(j).unwrap() {
                    flag = false;
                    break;
                }
            }
            if flag {
                return i as i32;
            }

            i += 1;
        }

        -1
    }
}
