use crate::Solution;

impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        let mut ans = Vec::new();

        for h in 0..12i32 {
            for m in 0..60i32 {
                if (h.count_ones() + m.count_ones()) as i32 == turned_on {
                    let tmp = if m < 10 { "0" } else { "" };
                    ans.push(format!("{}:{}{}", h, tmp, m));
                }
            }
        }

        ans
    }
}
