use crate::Solution;

impl Solution {
    pub fn find_lu_slength(a: String, b: String) -> i32 {
        if a != b {
            a.len().max(b.len()) as i32
        } else {
            -1
        }
    }
}
