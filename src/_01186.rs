use crate::Solution;

impl Solution {
    pub fn maximum_sum(arr: Vec<i32>) -> i32 {
        let mut dp0 = arr[0];
        let mut dp1 = 0;
        let mut ans = arr[0];

        for i in 1..arr.len() {
            dp1 = std::cmp::max(dp0, dp1 + arr[i]);
            dp0 = std::cmp::max(dp0, 0) + arr[i];
            ans = std::cmp::max(ans, dp0.max(dp1));
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_work() {
        assert_eq!(Solution::maximum_sum(vec![1, -2, 0, 3]), 4);
        assert_eq!(Solution::maximum_sum(vec![1, -2, -2, 3]), 3);
        assert_eq!(Solution::maximum_sum(vec![-1, -1, -1, -1]), -1);
    }
}
