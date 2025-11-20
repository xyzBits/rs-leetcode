use crate::Solution;

impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let mut number = 0;

        let mut multiple = 1;
        for i in (0..=column_title.len() - 1).rev() {
            let k = column_title.chars().nth(i).unwrap() as i32 - 'A' as i32 + 1;
            number += k * multiple;
            multiple *= 26;
        }

        number
    }
}
