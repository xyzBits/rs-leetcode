use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hash_table = HashMap::new();

        for i in 0..nums.len() {
            let remain = target - nums[i];

            if let Some(&remain_index) = hash_table.get(&remain) {
                return vec![remain_index, i as i32];
            }

            hash_table.insert(nums[i], i as i32);
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(Solution::two_sum(nums, target), vec![0, 1]);
    }

    #[test]
    fn test_2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        assert_eq!(Solution::two_sum(nums, target), vec![1, 2]);
    }

    #[test]
    fn test_3() {
        let nums = vec![3, 3];
        let target = 6;
        assert_eq!(Solution::two_sum(nums, target), vec![0, 1]);
    }
}
