use crate::Solution;
use std::collections::HashSet;

impl Solution {
    pub fn duplicate_numbers_xor(nums: Vec<i32>) -> i32 {
        let mut set = HashSet::new();

        let mut ans = 0;

        for &num in &nums {
            if set.contains(&num) {
                ans ^= num;
            } else {
                set.insert(num);
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::duplicate_numbers_xor(vec![1, 2, 1, 3]), 1);
        assert_eq!(Solution::duplicate_numbers_xor(vec![1, 2, 3]), 0);
        assert_eq!(Solution::duplicate_numbers_xor(vec![1, 2, 2, 1]), 3);
    }
}
