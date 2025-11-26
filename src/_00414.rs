use crate::Solution;

impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let min_value = i64::MIN;
        let (mut a, mut b, mut c) = (min_value, min_value, min_value);

        for num in nums {
            let num = num as i64;
            if num > a {
                c = b;
                b = a;
                a = num;
            } else if a > num && num > b {
                c = b;
                b = num;
            } else if b > num && num > c {
                c = num;
            }
        }

        if c == min_value {
            a as i32
        } else {
            c as _
        }
    }
}
