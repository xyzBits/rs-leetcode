use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            if let Some(vec_ref) = map.get_mut(&num) {
                vec_ref[0] += 1;
                vec_ref[2] = i as i32;
            } else {
                map.insert(num, vec![1, i as i32, i as i32]);
            }
        }

        let mut max_num = 0;
        let mut min_len = 0;
        for (key, arr) in map.iter() {
            if max_num < arr[0] {
                max_num = arr[0];
                min_len = arr[2] - arr[1] + 1;
            } else if max_num == arr[0] {
                if min_len > arr[2] - arr[1] + 1 {
                    min_len = arr[2] - arr[1] + 1;
                }
            }
        }
        min_len
    }
}
