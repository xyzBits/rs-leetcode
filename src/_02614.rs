use crate::Solution;

impl Solution {
    pub fn diagonal_prime(nums: Vec<Vec<i32>>) -> i32 {
        let n = nums.len();
        let mut res = 0;

        for i in 0..n {
            if Self::is_prime(nums[i][i]) {
                res = res.max(nums[i][i]);
            }

            if Self::is_prime(nums[i][n - i - 1]) {
                res = res.max(nums[i][n - i - 1]);
            }
        }

        res
    }

    fn is_prime(num: i32) -> bool {
        if num == 1 {
            return false;
        }

        let mut factor = 2;

        while factor * factor <= num {
            if num % factor == 0 {
                return false;
            }
            factor += 1;
        }

        true
    }
}
