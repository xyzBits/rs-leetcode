use crate::Solution;

impl Solution {
    pub fn break_palindrome(palindrome: String) -> String {
        let n = palindrome.len();
        if n == 1 {
            return "".to_string();
        }

        let mut data = palindrome.as_bytes().to_vec();

        let mut i = 0;
        while i * 2 + 1 < n {
            if data[i] != 'a' as u8 {
                data[i] = 'a' as u8;
                return String::from_utf8(data).unwrap();
            }

            i += 1;
        }

        data[n - 1] = 'b' as u8;
        String::from_utf8(data).unwrap()
    }
}
