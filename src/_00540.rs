use crate::Solution;

impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let (mut low, mut high) = (0_usize, nums.len() - 1);

        while low < high {
            let mid = (high - low) / 2 + low;
            if nums[mid] == nums[mid ^ 1] {
                low = mid + 1;
            } else {
                high = mid;
            }
        }

        nums[low]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::single_non_duplicate(vec![1, 1, 2, 3, 3, 4, 4, 8, 8]),
            2
        );
        assert_eq!(
            Solution::single_non_duplicate(vec![3, 3, 7, 7, 10, 11, 11]),
            10
        );
    }
}
