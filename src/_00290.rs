use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut str_2_ch = HashMap::new();
        let mut ch_2_str = HashMap::new();
        let m = s.len();
        let mut i = 0;
        let s_bytes = s.as_bytes();

        for ch in pattern.chars() {
            if i >= m {
                return false;
            }

            let mut j = i;
            while j < m && s_bytes[j] != b' ' {
                j += 1;
            }

            let tmp = &s[i..j];

            if let Some(&mapped_ch) = str_2_ch.get(tmp) {
                if mapped_ch != ch {
                    return false;
                }
            }

            if let Some(&mapped_str) = ch_2_str.get(&ch) {
                if mapped_str != tmp {
                    return false;
                }
            }

            str_2_ch.insert(tmp, ch);
            ch_2_str.insert(ch, tmp);
            i = j + 1
        }

        i >= m
    }
}

#[test]
fn test_001() {
    let data = "hello world".to_string();
    let bytes_data = data.as_bytes();
    assert_eq!(bytes_data[0], b'h');

    let mut str2ch = HashMap::new();
    str2ch.insert("hello", 'h');

    // &mapped_ch 把外面的那层引用剥掉，把里面的值赋给 mapped_ch
    // 解析引用
    if let Some(&mapped_ch) = str2ch.get("hello") {
        println!("{}", mapped_ch);
    }

    // 手动解引用
    if let Some(mapped_ch_ref) = str2ch.get("hello") {
        if *mapped_ch_ref == 'h' {
            println!("manual {}", mapped_ch_ref);
        }
    }
}
