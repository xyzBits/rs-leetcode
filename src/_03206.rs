use crate::Solution;

impl Solution {
    pub fn number_of_alternating_groups_(colors: Vec<i32>) -> i32 {
        let n = colors.len();
        let mut ans = 0;

        for i in 0..n {
            if colors[i] != colors[(i - 1 + n) % n] && colors[i] != colors[(i + 1) % n] {
                ans += 1;
            }
        }

        ans
    }
}
