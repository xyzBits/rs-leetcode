use crate::Solution;

impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        let mut score = 0;

        let bytes = s.as_bytes();
        for i in 0..bytes.len() - 1 {
            score += (bytes[i] as i32 - bytes[i + 1] as i32).abs();
        }

        score
    }
}

#[test]
fn test_01() {
    let data = "hello".to_string();
    println!("data.len={}", data.len());

    let bytes = data.as_bytes();
    println!("bytes.len={}", bytes.len());

    println!("{:?}", bytes);

    println!("char a byte: {:?}", bytes[0] as char);
    println!("char a byte: {:?}", 'h' as u8);

    println!("{:?}", 1 as char);
}
