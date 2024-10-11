use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        let mut count_1 = HashMap::new();
        let mut count_2 = HashMap::new();

        let mut max = 0;
        for &num in &nums1 {
            count_1.insert(num, count_1.get(&num).unwrap_or(&0) + 1);
            max = max.max(num);
        }

        for &num in &nums2 {
            count_2.insert(num, count_2.get(&num).unwrap_or(&0) + 1);
        }

        let mut res = 0;

        for &a in count_2.keys() {
            for b in ((a * k)..=max).step_by((a * k) as usize) {
                if let Some(&value) = count_1.get(&b) {
                    res += value as i64 * count_2.get(&a).unwrap_or(&0);
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums1 = vec![1, 3, 4];
        let nums2 = vec![1, 3, 4];
        let k = 1;
        assert_eq!(5, Solution::number_of_pairs(nums1, nums2, k));
    }

    #[test]
    fn test_2() {
        let nums1 = vec![1, 2, 4, 12];
        let nums2 = vec![2, 4];
        let k = 3;
        assert_eq!(2, Solution::number_of_pairs(nums1, nums2, k));
    }
}
