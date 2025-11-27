use crate::Solution;

impl Solution {
    pub fn count_segments(s: String) -> i32 {
        let mut segment_count = 0;

        for (i, ch) in s.char_indices() {
            if (i == 0 || s.chars().nth(i - 1) == Some(' ')) && ch != ' ' {
                segment_count += 1;
            }
        }

        segment_count
    }
}
