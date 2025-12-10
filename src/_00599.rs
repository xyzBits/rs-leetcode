use crate::Solution;
use std::collections::HashMap;
use std::usize;

impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let index: HashMap<_, _> = list1
            .iter()
            .into_iter()
            .enumerate()
            .map(|(index, element)| (element, index))
            .collect();

        let mut ret = Vec::new();
        let mut index_sum = usize::MAX;
        for (i, element) in list2.iter().enumerate() {
            if index.contains_key(&element) {
                let j = index[&element];
                if i + j < index_sum {
                    ret.clear();
                    ret.push(element.clone());
                    index_sum = i + j;
                } else if i + j == index_sum {
                    ret.push(element.clone());
                }
            }
        }

        ret
    }
}

#[test]
fn test_find_restaurant() {
    let data = vec!["hello", "world", "rust"];

    let map = data
        .iter()
        .enumerate()
        .map(|(i, element)| (*element, i))
        .collect::<HashMap<_, _>>();

    println!("{:?}", map);
}
