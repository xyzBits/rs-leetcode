use crate::Solution;

impl Solution {
    pub fn check_record(s: String) -> bool {
        let mut absents = 0;
        let mut lates = 0;

        for ch in s.chars() {
            if ch == 'A' {
                absents += 1;
                if absents >= 2 {
                    return false;
                }
            }

            if ch == 'L' {
                lates += 1;
                if lates >= 3 {
                    return false;
                }
            } else {
                lates = 0;
            }
        }

        true
    }
}
