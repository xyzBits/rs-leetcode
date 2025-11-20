use crate::Solution;

impl Solution {
    pub fn convert_to_title(mut column_number: i32) -> String {
        let mut result = String::new();
        while column_number > 0 {
            let a0 = (column_number - 1) % 26 + 1;
            result.push((a0 as u8 - 1 + b'A') as char);
            column_number = (column_number - a0) / 26
        }

        result.chars().rev().collect()
    }
}
