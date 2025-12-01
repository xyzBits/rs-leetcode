use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut map = HashMap::new();
        let mut stack = Vec::new();
        for &num in nums2.iter().rev() {
            while let Some(&top) = stack.last() {
                if num >= top {
                    stack.pop();
                } else {
                    break;
                }
            }
            map.insert(
                num,
                if let Some(&top) = stack.last() {
                    top
                } else {
                    -1
                },
            );
            stack.push(num);
        }

        nums1
            .iter()
            .map(|&num| *map.get(&num).unwrap_or(&-1))
            .collect()
    }
}
