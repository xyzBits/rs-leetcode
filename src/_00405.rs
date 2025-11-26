use crate::Solution;

impl Solution {
    pub fn to_hex(num: i32) -> String {
        // format!("{:x}", num);
        if num == 0 {
            return "0".to_owned();
        }

        let n = num as u32;
        let mut ans = String::new();
        for i in (0..=7).rev() {
            let val = (n >> (4 * i)) & 0xF;
            if !ans.is_empty() || val != 0 {
                let digit = match val {
                    0..=9 => (b'0' + val as u8) as char,
                    _ => (b'a' + (val - 10) as u8) as char,
                };
                ans.push(digit);
            }
        }

        ans
    }
}

#[test]
fn test_to_hex() {
    let borrowed_str = "Hello Rust";
    // 将借用的专为拥有所有权的
    let owned_by_owned = borrowed_str.to_owned();

    // 内容格式化为 string
    let owned_by_string = borrowed_str.to_string();

    let num = 12345;
    let s_result = num.to_string();

    let borrowed_slice: &[i32; 3] = &[1, 2, 3];
    let owned_vec = borrowed_slice.to_owned();

    let vec = borrowed_slice.to_vec();
}
