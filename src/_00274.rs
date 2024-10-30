use crate::Solution;

impl Solution {
    pub fn h_index(mut citations: Vec<i32>) -> i32 {
        citations.sort();

        let n = citations.len();
        for i in (1..=n).rev() {
            if citations[n - i] >= i as i32 {
                return i as i32;
            }
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_h_index() {
        assert_eq!(Solution::h_index(vec![3, 0, 6, 1, 5]), 3);
        assert_eq!(Solution::h_index(vec![1, 3, 1]), 1);
    }
}
