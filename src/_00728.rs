use crate::Solution;

impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let mut ans = vec![];

        for i in left..=right {
            if Self::is_self_dividing(i) {
                ans.push(i);
            }
        }

        ans
    }

    fn is_self_dividing(num: i32) -> bool {
        let mut temp = num;
        while temp > 0 {
            let digit = temp % 10;
            if digit == 0 || num % digit != 0 {
                return false;
            }
            temp /= 10;
        }

        true
    }
}
