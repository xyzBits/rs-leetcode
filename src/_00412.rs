use crate::Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut answer = Vec::new();

        for i in 1..=n {
            let mut tmp = String::new();
            if i % 3 == 0 {
                tmp.push_str("Fizz");
            }

            if i % 5 == 0 {
                tmp.push_str("Buzz");
            }

            if tmp.is_empty() {
                tmp.push_str(i.to_string().as_str());
            }

            answer.push(tmp);
        }

        answer
    }
}
