use crate::Solution;
use std::collections::BinaryHeap;

impl Solution {
    pub fn max_removal(nums: Vec<i32>, mut queries: Vec<Vec<i32>>) -> i32 {
        queries.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut heap = BinaryHeap::new();
        let mut delta_array = vec![0; nums.len() + 1];
        let mut operations = 0;
        let mut j = 0;

        for i in 0..nums.len() {
            operations += delta_array[i];
            while j < queries.len() && queries[j][0] == i as i32 {
                heap.push(queries[j][1]);
                j += 1;
            }
            while operations < nums[i] && !heap.is_empty() && *heap.peek().unwrap() >= i as i32 {
                operations += 1;
                let end = heap.pop().unwrap() as usize;
                delta_array[end + 1] -= 1;
            }
            if operations < nums[i] {
                return -1;
            }
        }
        heap.len() as i32
    }
}
