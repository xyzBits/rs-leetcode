use crate::Solution;

impl Solution {
    pub fn num_friend_requests(mut ages: Vec<i32>) -> i32 {
        let n = ages.len();
        ages.sort();
        let (mut left, mut right, mut ans) = (0, 0, 0);

        for &age in &ages {
            if age < 15 {
                continue;
            }

            while ages[left] as f64 <= 0.5 * age as f64 + 7f64 {
                left += 1;
            }

            while right + 1 < n && ages[right + 1] <= age {
                right += 1;
            }
            ans += right - left;
        }

        ans as i32
    }
}
