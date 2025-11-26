use crate::Solution;

impl Solution {
    pub fn add_strings_(num1: String, num2: String) -> String {
        let (mut i, mut j, mut add) = (num1.len() - 1, num2.len() - 1, 0);
        let mut ans = String::new();

        while i > 0 || j > 0 || add != 0 {
            let x = if i > 0 {
                num1.chars().nth(i).unwrap_or('0') as u8 - b'0'
            } else {
                0
            };

            let y = if j > 0 {
                num2.chars().nth(j).unwrap_or('0') as u8 - b'0'
            } else {
                0
            };

            let result = x as i32 + y as i32 + add;
            ans.push_str(&format!("{}", result % 10));
            add = result / 10;
            i -= 1;
            j -= 1;
        }
        ans.chars().rev().collect()
    }

    pub fn add_strings(num1: String, num2: String) -> String {
        let n1_bytes = num1.as_bytes();
        let n2_bytes = num2.as_bytes();

        let mut i = n1_bytes.len() as isize - 1;
        let mut j = n2_bytes.len() as isize - 1;
        let mut carry = 0_i32;

        let mut ans_digits = Vec::<char>::new();

        while i >= 0 || j >= 0 || carry != 0 {
            let x = if i >= 0 {
                (n1_bytes[i as usize] - b'0') as i32
            } else {
                0
            };

            let y = if j >= 0 {
                (n2_bytes[j as usize] - b'0') as i32
            } else {
                0
            };

            let result = x + y + carry;
            ans_digits.push((b'0' + (result % 10) as u8) as char);

            carry = result / 10;

            i -= 1;
            j -= 1;
        }

        ans_digits.into_iter().rev().collect()
    }
}

#[test]
fn test_add_strings() {
    let data = vec![
        "a".to_owned(),
        "b".to_owned(),
        "c".to_owned(),
        "d".to_owned(),
    ];
    for s_ref in data.iter() {
        println!("{}", s_ref);
    }

    println!("{:?}", data);

    for s_owned in data.into_iter() {
        println!("{}", s_owned);
    }
    // into_iter 已经消耗掉了所有权
    // println!("{:?}", data);
}

#[test]
fn test_002() {
    // for_each 无法中断
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    data.iter()
        .filter(|&x| x > &3)
        .map(|&x| x * 2)
        .for_each(|x| println!("{}", x));
}
