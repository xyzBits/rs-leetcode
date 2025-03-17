use crate::Solution;

impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let (mut cnt, mut min_cnt) = (0, 0);

        for &byte in s.as_bytes() {
            if byte == b'[' {
                cnt += 1;
            } else {
                cnt -= 1;
                min_cnt = min_cnt.min(cnt);
            }
        }

        (-min_cnt + 1) / 2
    }
}
