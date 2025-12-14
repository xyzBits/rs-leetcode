use crate::Solution;

impl Solution {
    pub fn can_be_valid(s: String, locked: String) -> bool {
        let n = s.len();
        let mut mx = 0;
        let mut mn = 0;

        let s_bytes = s.as_bytes();
        let locked_bytes = locked.as_bytes();
        for i in 0..n {
            if locked_bytes[i] == b'(' {
                let diff;
                if s_bytes[i] == b'(' {
                    diff = 1;
                } else {
                    diff = -1;
                }
                mx += diff;
                mn = (mn + diff).max((i as i32 + 1) % 2);
            } else {
                mx += 1;
                mn = (mn - 1).max((i as i32 + 1) % 2);
            }

            if mx < mn {
                return false;
            }
        }

        mn == 0
    }
}
