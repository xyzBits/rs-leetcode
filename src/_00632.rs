use crate::Solution;
use std::collections::BinaryHeap;

impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut range_left = 0;
        let mut range_right = i32::MAX;
        let min_range = range_right - range_left;

        let mut max_value = i32::MIN;
        let size = nums.len();
        let mut next = vec![0; size];

        let mut priority_queue = BinaryHeap::new();

        for i in 0..size {
            priority_queue.push(std::cmp::Reverse((nums[i][0], i)));
            max_value = max_value.max(nums[i][0]);
        }

        while let Some(std::cmp::Reverse((min_value, row))) = priority_queue.pop() {
            if max_value - min_value < range_right - range_left {
                range_left = min_value;
                range_right = max_value;
            }

            if next[row] == nums[row].len() - 1 {
                break;
            }

            next[row] += 1;
            max_value = max_value.max(nums[row][next[row]]);
            priority_queue.push(std::cmp::Reverse((nums[row][next[row]], row)));
        }

        vec![range_left, range_right]
    }
}
