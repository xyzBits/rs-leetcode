use crate::Solution;
use std::collections::{BTreeMap, HashMap};

impl Solution {
    pub fn beautiful_subsets(nums: Vec<i32>, k: i32) -> i32 {
        let mut groups = HashMap::new();

        for &a in &nums {
            let mod_val = a % k;
            groups
                .entry(mod_val)
                .or_insert(BTreeMap::new())
                .entry(a)
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }

        let mut ans = 1;

        for group in groups.values() {
            let m = group.len();
            let mut f = vec![vec![0; 2]; m];
            let first_key = *group.keys().next().unwrap();
            f[0][0] = 1;
            f[0][1] = (1 << group[&first_key]) - 1;

            let mut i = 1;
            let mut prev_key = first_key;

            for (&curr_key, &count) in group.iter().skip(1) {
                f[i][0] = f[i - 1][0] + f[i - 1][1];
                if curr_key - prev_key == k {
                    f[i][1] = f[i - 1][0] * ((1 << count) - 1);
                } else {
                    f[i][1] = (f[i - 1][0] + f[i - 1][1]) * ((1 << count) - 1);
                }

                prev_key = curr_key;
                i += 1;
            }
            ans *= f[m - 1][0] + f[m - 1][1];
        }

        ans - 1
    }
}

#[test]
fn test_01() {
    let mut map = HashMap::new();
    map.insert("apple", 3);

    map.insert("apple", 5);

    if let Some(value) = map.get_mut("apple") {
        *value = 7;
    }

    println!("{:?}", map);

    // 存在才修改
    map.entry("apple").and_modify(|v| *v *= 2);

    println!("{:?}", map);

    // 若不存在则插入默认值
    map.entry("banana").and_modify(|v| *v += 1).or_insert(98);
    println!("{:?}", map);
}

#[test]
fn test_02() {
    let mut map = HashMap::new();
    map.insert("apple", 3);
    map.insert("banana", 5);
    map.insert("orange", 7);

    for (key, value) in map.iter() {
        println!("{}: {}", key, value);
    }

    for (key, value) in map.iter_mut() {
        *value += 1;
        println!("{}: {}", key, value);
    }

    let transformed = map
        .iter()
        .map(|(k, &v)| (k.to_uppercase(), v * 2))
        .collect::<HashMap<_, _>>();

    println!("{:?}", transformed);
}

#[test]
fn test_03() {
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    for num in vec.iter() {
        println!("{}", num);
    }
}

#[test]
fn test_04() {
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    for num in vec.into_iter() {
        println!("{}", num);
    }
}
