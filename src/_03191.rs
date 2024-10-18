use crate::Solution;

impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut ans = 0;

        for i in 0..len {
            if nums[i] == 0 {
                if i > len - 3 {
                    return -1;
                }

                nums[i] ^= 1;
                nums[i + 1] ^= 1;
                nums[i + 2] ^= 1;
                ans += 1;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::min_operations(vec![0, 1, 1, 1, 0, 0]), 3);
        assert_eq!(Solution::min_operations(vec![0, 1, 1, 1]), -1);
    }
}
