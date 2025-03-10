use crate::Solution;

impl Solution {
    pub fn divisor_substrings(num: i32, k: i32) -> i32 {
        let str_num = num.to_string();
        let n = str_num.len();

        let mut ans = 0;

        for i in 0..=n - k as usize {
            let tmp = str_num[i..i + k as usize].parse::<i32>().unwrap();
            if tmp != 0 && num % tmp == 0 {
                ans += 1;
            }
        }

        ans
    }
}
