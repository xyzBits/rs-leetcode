use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut error_nums = vec![0; 2];
        let n = nums.len();
        let mut map = HashMap::new();
        for num in nums {
            *map.entry(num).or_insert(0) += 1;
        }

        for i in 1..=n as i32 {
            let count = map.get(&i).unwrap_or(&0);
            if *count == 2 {
                error_nums[0] = i;
            } else if &0 == count {
                error_nums[1] = i;
            }
        }

        error_nums
    }
}

#[test]
fn test_001() {
    assert_eq!(Some("car").unwrap_or("bike"), "car");
    assert_eq!(None.unwrap_or("bike"), "bike");
}
