use crate::Solution;

impl Solution {
    pub fn count_complete_day_pairs(hours: Vec<i32>) -> i32 {
        let mut ans = 0;

        for i in 1..hours.len() {
            for j in 0..i {
                if (hours[i] + hours[j]) % 24 == 0 {
                    ans += 1;
                }
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::count_complete_day_pairs(vec![12, 12, 30, 24, 24]),
            2
        );
        assert_eq!(Solution::count_complete_day_pairs(vec![72, 48, 24, 3]), 3);
    }
}
