use crate::Solution;

impl Solution {
    pub fn convert_to_base7(mut num: i32) -> String {
        if num == 0 {
            return "0".to_owned();
        }

        let negative = num < 0;
        num = num.abs();
        let mut digits = String::new();

        while num > 0 {
            let remainder = num % 7;

            // 将数字余数转换为字符 '0'-'6' 并推入 Vec
            digits.push(std::char::from_digit(remainder as u32, 10).unwrap());
            num /= 7;
        }

        if negative {
            digits.push('-');
        }

        digits.chars().rev().collect::<String>()
    }
}
