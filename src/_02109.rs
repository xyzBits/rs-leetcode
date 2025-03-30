use crate::Solution;

impl Solution {
    pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
        let mut ans = Vec::new();
        let mut j = 0;

        for i in 0..s.len() {
            if j < spaces.len() && spaces[j] == i as i32 {
                ans.push(b' ');
                j += 1;
            }
            ans.push(s.as_bytes()[i]);
        }

        String::from_utf8(ans).unwrap()
    }
}
