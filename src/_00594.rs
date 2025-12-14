use std::cell::RefCell;
use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut cnt = HashMap::new();
        let mut res = 0;

        for num in nums {
            let value = cnt.entry(num).or_insert(0);
            *value += 1;
        }

        for &key in cnt.keys() {
            if cnt.contains_key(&(key + 1)) {
                res = res.max(cnt[&key] + cnt[&(key + 1)]);
            }
        }

        res
    }
}

#[test]
fn test_001() {
    let mut letters = HashMap::new();
    for ch in "a short treatise on fungi".chars() {
        letters
            .entry(ch)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    assert_eq!(letters[&'s'], 2);
    assert_eq!(letters[&'t'], 3);
    assert_eq!(letters[&'u'], 1);

    assert_eq!(letters.get(&'y'), None);
}

#[test]
fn test_002() {
    let mut map = HashMap::new();

    map.entry("poneyland").or_insert(3);
    println!("{:?}", map);
    // 不存在的话才 insert
    map.entry("poneyland").or_insert(4);
    println!("{:?}", map);

    let value = map.entry("poneyland").or_insert(5);

    *value *= 2;
    println!("{:?}", map);

    map.entry("hello")
        .and_modify(|value| *value += 1)
        .or_insert(299);

    println!("{:?}", map);

    map.entry("hello")
        .and_modify(|value| *value += 1)
        .or_insert(1);
    println!("{:?}", map);

    // 在一个表达工中处理 key 的存在和不存在两种情况
    // 存在 and_modify  执行
    // 不存在 or_insert 执行
}

#[test]
fn test_003() {
    let mut scores = HashMap::new();

    scores.insert("Alice".to_string(), 85);
    scores.insert("Bob".to_string(), 50);

    let target_user = "Alice";
    let new_user = "Charlie";

    scores
        .entry(target_user.to_string())
        .and_modify(|score| {
            println!("正在修改 {} 的分数 {}", target_user, score);
            *score += 10;
        })
        .or_insert(0);

    scores
        .entry(new_user.to_string())
        .and_modify(|score| {
            *score += 10;
        })
        .or_insert(70);

    println!("{:?}", scores);
}

#[test]
fn test_004() {
    use std::rc::Rc;

    let data = Rc::new(vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h']);
    let size = (&data).len();
    println!("{}", size);

    let data = Box::new(vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h']);
    let size = (&data).len();
    println!("{}", size);
    println!("{}", data.len());

    let data = Rc::new(RefCell::new(5));
    let mut ref_mut = data.borrow_mut();
    *ref_mut += 1;
    println!("{:?}", data);
}
