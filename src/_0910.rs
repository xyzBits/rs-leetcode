use crate::Solution;

impl Solution {
    pub fn smallest_range_ii(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();

        let len = nums.len();

        let mi = nums[0];
        let ma = nums[len - 1];

        let mut ans = ma - mi;

        for i in 0..len - 1 {
            let a = nums[i];
            let b = nums[i + 1];

            ans = ans.min((ma - k).max(a + k) - (mi + k).min(b - k));
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::smallest_range_ii(vec![1], 0), 0);
        assert_eq!(Solution::smallest_range_ii(vec![0, 10], 2), 6);
        assert_eq!(Solution::smallest_range_ii(vec![1, 3, 6], 3), 3);
    }
}
