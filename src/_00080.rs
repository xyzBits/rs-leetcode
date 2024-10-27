use crate::Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let n = nums.len();
        if n <= 2 {
            return n as i32;
        }

        let (mut slow, mut fast) = (2, 2);

        while fast < n {
            if nums[slow - 2] != nums[fast] {
                nums[slow] = nums[fast];
                slow += 1;
            }

            fast += 1;
        }

        slow as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_remove_duplicates() {
        assert_eq!(Solution::remove_duplicates(&mut vec![1, 1, 1, 2, 2, 3]), 5);
        assert_eq!(
            Solution::remove_duplicates(&mut vec![0, 0, 1, 1, 1, 1, 2, 3, 3]),
            7
        );
    }
}
