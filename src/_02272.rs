use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn largest_variance(s: String) -> i32 {
        let mut pos = HashMap::new();

        for (i, ch) in s.char_indices() {
            pos.entry(ch).or_insert(Vec::new()).push(i);
        }

        let mut ans = 0;

        for (&c0, pos0) in &pos {
            for (&c1, pos1) in &pos {
                if c0 != c1 {
                    let (mut i, mut j) = (0, 0);
                    let (mut f, mut g) = (0, i32::MIN);

                    while i < pos0.len() || j < pos1.len() {
                        if j == pos1.len() || (i < pos0.len() && pos0[i] < pos1[j]) {
                            f = f.max(0) + 1;
                            g += 1;
                            i += 1;
                        } else {
                            g = f.max(g.max(0)) - 1;
                            f = f.max(0) - 1;
                            j += 1;
                        }
                        ans = ans.max(g)
                    }
                }
            }
        }

        ans
    }
}

#[test]
fn test_01() {
    let data = "hello world".to_string();

    for ch in data.chars() {}

    for (i, ch) in data.char_indices() {}

    for ch in data.bytes() {}

    let mut map = HashMap::new();
    map.insert(1, "hello world".to_string());
    map.insert(2, "goodbye".to_string());

    for (key, value) in &map {}

    let value = &map[&1];
    println!("{}", value);
}
