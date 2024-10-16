use crate::Solution;

impl Solution {
    pub fn count_alternating_subarrays(nums: Vec<i32>) -> i64 {
        let (mut ans, mut cur, mut prev) = (0, 0, -1);

        for &num in &nums {
            cur = if prev != num { cur + 1 } else { 1 };
            prev = num;
            ans += cur;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::count_alternating_subarrays(vec![0, 1, 1, 1]), 5);
        assert_eq!(Solution::count_alternating_subarrays(vec![1, 0, 1, 0]), 10);

        let data = vec![1, 2, 3, 4];

        let mut data = data;

        data.push(3);
        println!("{:?}", data);
    }
}
