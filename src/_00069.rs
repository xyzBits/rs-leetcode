use crate::Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let (mut left, mut right, mut ans) = (0, x, -1);

        while left <= right {
            let mid = left + (right - left) / 2;
            if mid as u64 * mid as u64 <= x as u64 {
                ans = mid;
                left = mid + 1;
            } else {
                right = mid - 1;
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
        let x = 4;
        assert_eq!(Solution::my_sqrt(x), 2);
    }

    #[test]
    fn test_2() {
        let x = 8;
        assert_eq!(Solution::my_sqrt(x), 2);
    }

    #[test]
    fn test_3() {
        let x = 2147395599;
        assert_eq!(Solution::my_sqrt(x), 46339);
    }
}
