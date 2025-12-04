use crate::Solution;

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        if word.len() >= 2
            && word.chars().nth(0).unwrap().is_lowercase()
            && word.chars().nth(1).unwrap().is_uppercase()
        {
            return false;
        }

        for i in 2..word.len() {
            if word.chars().nth(i).unwrap().is_lowercase()
                ^ word.chars().nth(1).unwrap().is_lowercase()
            {
                return false;
            }
        }
        true
    }
}
