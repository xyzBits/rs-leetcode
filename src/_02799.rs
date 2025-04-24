use crate::Solution;
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
        let mut res = 0;

        let mut cnt: HashMap<i32, i32> = HashMap::new();
        let n = nums.len();
        let mut right = 0;
        let distinct = nums.iter().collect::<HashSet<_>>().len();

        for left in 0..n {
            if left > 0 {
                let remove = &nums[left - 1];
                *cnt.get_mut(remove).unwrap() -= 1;

                if cnt[remove] == 0 {
                    cnt.remove(remove);
                }
            }

            while right < n && cnt.len() < distinct {
                let add = &nums[right];
                *cnt.entry(*add).or_insert(0) += 1;
                right += 1;
            }

            if cnt.len() == distinct {
                res += (n - right + 1) as i32;
            }
        }

        res
    }
}
