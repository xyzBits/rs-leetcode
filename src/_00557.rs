use crate::Solution;

impl Solution {
    pub fn reverse_words_(s: String) -> String {
        let mut ret = String::new();

        let length = s.len();
        let mut i = 0;
        while i < length {
            let start = i;
            while i < length && s.chars().nth(i) != Some(' ') {
                i += 1;
            }

            for p in start..i {
                ret.push(s.chars().nth(start + i - 1 - p).unwrap());
            }

            while i < length && s.chars().nth(i) == Some(' ') {
                i += 1;
                ret.push(' ');
            }
        }

        ret
    }

    pub fn reverse_words(s: String) -> String {
        let length = s.len();
        let mut i = 0;

        let mut chars_vec = s.chars().collect::<Vec<char>>();
        while i < length {
            let start = i;
            while i < length && chars_vec[i] != ' ' {
                i += 1;
            }

            let mut left = start;
            let mut right = i - 1;
            while left < right {
                chars_vec.swap(left, right);
                left += 1;
                right -= 1;
            }

            while i < length && chars_vec[i] == ' ' {
                i += 1;
            }
        }

        chars_vec.iter().collect::<String>()
    }
}

#[test]
fn test_001() {
    let mut data = vec!["hello", " ", "world", "!"];
    data.swap(1, 2);
    println!("{:?}", data);
}
