use crate::Solution;

impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let mut chars = dominoes.chars().collect::<Vec<char>>();

        let n = dominoes.len();
        let mut i = 0;

        let mut left = 'L';

        while i < n {
            let mut j = i;
            while j < n && chars[j] == '.' {
                j += 1;
            }

            let right = if j < n { chars[j] } else { 'R' };
            if left == right {
                while i < j {
                    chars[i] = right;
                    i += 1;
                }
            } else if left == 'R' && right == 'L' {
                let mut k = j - 1;
                while i < k {
                    chars[i] = 'R';
                    i += 1;

                    chars[k] = 'L';
                    k -= 1;
                }
            }

            left = right;
            i = j + 1;
        }

        chars.into_iter().collect()
    }
}
