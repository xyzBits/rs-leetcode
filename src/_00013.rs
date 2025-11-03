use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut symbol_values = HashMap::new();
        symbol_values.insert('I', 1);
        symbol_values.insert('V', 5);
        symbol_values.insert('X', 10);
        symbol_values.insert('L', 50);
        symbol_values.insert('C', 100);
        symbol_values.insert('D', 500);
        symbol_values.insert('M', 1000);

        let mut ans = 0;

        // map list [] 都实现了 Index trait
        let n = s.len();
        for i in 0..n {
            let value = symbol_values[&s.chars().nth(i).unwrap()];
            if i < n - 1 && value < symbol_values[&s.chars().nth(i + 1).unwrap()] {
                ans -= value;
            } else {
                ans += value;
            }
        }

        ans
    }
}
