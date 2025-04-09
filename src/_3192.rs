use crate::Solution;

impl Solution {
    pub fn min_operations_1(nums: Vec<i32>) -> i32 {
        let mut operation = 0;

        for &num in &nums {
            if num == (operation % 2) {
                operation += 1;
            }
        }

        operation
    }

    pub fn min_operations_3192(nums: Vec<i32>) -> i32 {
        let mut operation = 0;

        for i in (0..nums.len() - 1).step_by(1).rev() {
            if nums[i] != nums[i + 1] {
                operation += 1;
            }
        }

        if nums[0] == 1 {
            operation
        } else {
            operation + 1
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::min_operations(vec![0, 1, 1, 0, 1]), 4);
        assert_eq!(Solution::min_operations(vec![1, 0, 0, 0]), 1);
    }
}
