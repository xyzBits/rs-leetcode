use crate::Solution;

use std::cmp::{max, min};

impl Solution {
    pub fn min_moves_to_capture_the_queen(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> i32 {
        // 车与皇后处在同一行，且中间没有象
        if a == e && (c != a || d <= min(b, f) || d >= max(b, f)) {
            return 1;
        }
        // 车与皇后处在同一列，且中间没有象
        if b == f && (d != b || c <= min(a, e) || c >= max(a, e)) {
            return 1;
        }
        // 象、皇后处在同一条对角线，且中间没有车
        if (c - e).abs() == (d - f).abs()
            && ((c - e) * (b - f) != (a - e) * (d - f) || a < min(c, e) || a > max(c, e))
        {
            return 1;
        }
        2
    }
}
