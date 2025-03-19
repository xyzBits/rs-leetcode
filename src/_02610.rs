use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut cnt = HashMap::new();
        for &num in &nums {
            *cnt.entry(num).or_insert(0) += 1;
        }

        let mut ans = Vec::new();
        while !cnt.is_empty() {
            let mut arr = Vec::new();

            let keys = cnt.keys().cloned().collect::<Vec<_>>();
            for key in keys {
                if let Some(value) = cnt.get_mut(&key) {
                    *value -= 1;
                    arr.push(key);
                    if *value == 0 {
                        cnt.remove(&key);
                    }
                }
            }

            ans.push(arr);
        }

        ans
    }
}

#[test]
fn test_01() {
    let mut map = HashMap::new();

    map.insert("one".to_string(), 1);
    map.insert("two".to_string(), 2);
    map.insert("three".to_string(), 3);

    if let Some(value) = map.get_mut("one") {
        *value = 10;
    }

    println!("{:?}", map);

    match map.remove("two") {
        Some(value) => {
            println!("deleted value: {}", value);
        }

        None => {
            println!("key not found");
        }
    }
}
