use crate::Solution;
use std::collections::HashSet;

impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        // let mut set = HashSet::new();
        // candy_type.iter().for_each(|&item| {
        //     set.insert(item);
        // });

        let set = candy_type.iter().collect::<HashSet<_>>();

        set.len().min(candy_type.len() / 1) as i32
    }
}
