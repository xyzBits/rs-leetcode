use crate::Solution;

impl Solution {
    pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let mut ans = vec![-1; n - k as usize + 1];

        for i in 0..n - k as usize + 1 {
            let mut valid = true;
            for j in i + 1..i + k as usize {
                if nums[j] - nums[j - 1] != 1 {
                    valid = false;
                    break;
                }
            }
            if valid {
                ans[i] = nums[i + k as usize - 1];
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_results_array() {
        assert_eq!(
            Solution::results_array(vec![1, 2, 3, 4, 3, 2, 5], 3),
            vec![3, 4, -1, -1, -1]
        );

        assert_eq!(
            Solution::results_array(vec![2, 2, 2, 2, 2], 4),
            vec![-1, -1]
        );

        assert_eq!(
            Solution::results_array(vec![3, 2, 3, 2, 3, 2], 2),
            vec![-1, 3, -1, 3, -1]
        );
    }
}
