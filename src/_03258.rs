use crate::Solution;

impl Solution {
    pub fn count_k_constraint_substrings_(s: String, k: i32) -> i32 {
        let n = s.len();
        let mut ans = 0;

        for i in 0..n {
            let mut count = vec![0; 2];
            for j in i..n {
                count[s.chars().nth(j).unwrap() as usize - '0' as usize] += 1;
                if count[0] > k && count[1] > k {
                    break;
                }

                ans += 1;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_count_k_constraint_substring() {
        assert_eq!(
            Solution::count_k_constraint_substrings_("10101".into(), 1),
            12
        );
        assert_eq!(
            Solution::count_k_constraint_substrings_("1010101".into(), 2),
            25
        );
        assert_eq!(
            Solution::count_k_constraint_substrings_("11111".into(), 1),
            15
        );
    }
}
