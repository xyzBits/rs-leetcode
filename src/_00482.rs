use crate::Solution;

impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let mut ans = String::new();

        let mut cnt = 0;
        for ch in s.chars().rev() {
            if ch != '-' {
                cnt += 1;
                ans.push_str(ch.to_uppercase().to_string().as_str());
                if cnt % k == 0 {
                    ans.push('-');
                }
            }
        }

        if ans.len() > 0 && ans.chars().nth(ans.len() - 1) == Some('-') {
            ans.pop();
        }

        ans.chars().rev().collect()
    }
}
