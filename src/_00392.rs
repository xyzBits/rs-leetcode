use crate::Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.is_empty() {
            return true;
        }
        let (n, m) = (s.len(), t.len());

        if n > m {
            return false;
        }

        let mut f = vec![vec![0; 26]; m + 1];
        for j in 0..26 {
            f[m][j] = m as i32;
        }

        let t_bytes = t.as_bytes();

        for i in (0..=m - 1).rev() {
            let current_char_idx = (t_bytes[i] - b'a') as usize;

            for j in 0..26 {
                if j == current_char_idx {
                    f[i][j] = i as i32;
                } else {
                    f[i][j] = f[i + 1][j];
                }
            }
        }

        let s_bytes = s.as_bytes();
        let mut add = 0_i32;
        for i in 0..n {
            let s_char_idx = (s_bytes[i] - b'a') as usize;

            let add_usize = add as usize;

            if f[add_usize][s_char_idx] == m as i32 {
                return false;
            }

            add = f[add_usize][s_char_idx] + 1;
        }

        true
    }
}
