use crate::Solution;

impl Solution {
    pub fn non_special_count(l: i32, r: i32) -> i32 {
        let n = (r as f64).sqrt().ceil() as usize;
        let mut v = vec![0; n + 1];

        let mut ans = r - l + 1;

        for i in 2..=n {
            if v[i] == 0 {
                if i as i32 * i as i32 >= l && i as i32 * i as i32 <= r {
                    ans -= 1;
                }
                for j in (i * 2..=n).step_by(i) {
                    v[j] = 1;
                }
            }
        }

        ans
    }
}
