use crate::Solution;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let mx = (&nums).iter().max().unwrap();
        let mut ans = 0_i64;

        let (mut cnt, mut left) = (0, 0);

        for &x in &nums {
            if x == *mx {
                cnt += 1;
            }

            while cnt == k {
                if nums[left] == *mx {
                    cnt -= 1;
                }
                left += 1;
            }
            ans += left as i64;
        }

        ans
    }
}
