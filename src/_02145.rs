use crate::Solution;

impl Solution {
    pub fn number_of_arrays(difference: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let (mut x, mut y, mut cur) = (0, 0, 0);

        for &dif in &difference {
            cur += dif;
            x = x.min(cur);
            y = y.max(cur);
            if y - x > upper - lower {
                return 0;
            }
        }

        (upper - lower) - (y - x) + 1
    }
}
