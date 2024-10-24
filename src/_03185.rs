use crate::Solution;

impl Solution {
    pub fn count_complete_day_pairs(hours: Vec<i32>) -> i64 {
        let mut ans = 0;

        let mut cnt = vec![0; 24];
        for hour in hours {
            ans += cnt[((24 - hour % 24) % 24) as usize];
            cnt[(hour % 24) as usize] += 1;
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