use crate::Solution;

impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut result = 0;
        let mut points = Vec::new();
        for op in operations {
            let n = points.len();
            match op.chars().nth(0) {
                Some('+') => {
                    result += points[n - 1] + points[n - 2];
                    points.push(points[n - 1] + points[n - 2]);
                }
                Some('D') => {
                    result += 2 * points[n - 1];
                    points.push(2 * points[n - 1]);
                }
                Some('C') => {
                    result -= points[n - 1];
                    points.remove(n - 1);
                }

                _ => {
                    result += op.parse::<i32>().unwrap();
                    points.push(op.parse::<i32>().unwrap());
                }
            }
        }

        result
    }
}
