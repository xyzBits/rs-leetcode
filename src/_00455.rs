use crate::Solution;

impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort_unstable();
        s.sort_unstable();
        let (m, n) = (g.len(), s.len());

        let mut count = 0;
        let (mut i, mut j) = (0, 0);
        while i < m && j < n {
            while j < n && g[i] > s[j] {
                j += 1;
            }

            if j < n {
                count += 1;
            }

            i += 1;
            j += 1;
        }

        count
    }
}
