use crate::Solution;

impl Solution {
    pub fn add_binary(mut a: String, mut b: String) -> String {
        // (a.parse::<i128>().unwrap() + b.parse::<i128>().unwrap()).to_string()

        let max_len = a.len().max(b.len());
        let mut result = String::new();
        let mut carry = 0;

        for _ in 0..max_len {
            let bit_a = a.pop().unwrap_or('0').to_digit(2).unwrap();
            let bit_b = b.pop().unwrap_or('0').to_digit(2).unwrap();

            if carry == 0 {
                if bit_a + bit_b == 2 {
                    result.push('0');
                    carry = 1;
                } else {
                    result.push_str(&(bit_a + bit_b).to_string());
                }
            } else {
                if bit_a + bit_b + carry == 3 {
                    result.push('1');
                    carry = 1;
                } else if bit_a + bit_b + carry == 2 {
                    result.push('0');
                    carry = 1;
                } else {
                    result.push_str(&(bit_a + bit_b + carry).to_string());
                    carry = 0;
                }
            }
        }

        if carry == 1 {
            result.push('1');
        }

        result.chars().rev().collect()
    }
}


#[test]
fn test_001() {
    let mut a = String::from("101");
    let bit_a = a.pop().unwrap_or('0').to_digit(2).unwrap();

    println!("{}", bit_a);
}