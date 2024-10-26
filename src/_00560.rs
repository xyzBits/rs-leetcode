use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        let mut pre = 0;

        let mut map = HashMap::new();
        map.insert(0, 1);

        for i in 0..nums.len() {
            pre += nums[i];

            if let Some(&value) = map.get(&(pre - k)) {
                count += value;
            }

            *map.entry(pre).or_insert(0) += 1;
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    use std::collections::HashMap;

    #[test]
    fn test() {
        assert_eq!(Solution::subarray_sum(vec![1, 1, 1], 2), 2);
        assert_eq!(Solution::subarray_sum(vec![1, 2, 3], 3), 2);

        let mut map = HashMap::new();

        map.entry("hello").or_insert(3);
        assert_eq!(map["hello"], 3);

        *map.entry("world").or_insert(0) += 1;
        assert_eq!(map["world"], 1);
    }
}
