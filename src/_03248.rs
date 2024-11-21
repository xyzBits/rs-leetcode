use crate::Solution;

impl Solution {
    pub fn final_position_of_snake(n: i32, commands: Vec<String>) -> i32 {
        let mut ans = 0;

        for cmd in commands {
            if cmd.chars().nth(0).unwrap() == 'U' {
                ans -= n;
            } else if cmd.chars().nth(0).unwrap() == 'D' {
                ans += n;
            } else if cmd.chars().nth(0).unwrap() == 'L' {
                ans -= 1;
            } else {
                ans += 1;
            }
        }

        ans
    }
}
