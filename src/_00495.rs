use crate::Solution;

impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        let mut ans = 0;

        let mut expired = 0;
        for time in time_series {
            if time >= expired {
                ans += duration;
            } else {
                ans += time + duration - expired;
            }

            expired = time + duration;
        }

        ans
    }
}
