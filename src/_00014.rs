use crate::Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "".to_string();
        }

        let length = strs[0].len();
        let count = strs.len();
        for i in 0..length {
            let c = strs[0].chars().nth(i).unwrap();

            for j in 1..count {
                if i == strs[j].len() || strs[j].chars().nth(i) != Some(c) {
                    return strs[0][0..i].to_string();
                }
            }
        }

        strs[0].to_string()
    }
}
