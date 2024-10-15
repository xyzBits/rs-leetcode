use crate::Solution;

impl Solution {
    pub fn max_height_of_triangle(red: i32, blue: i32) -> i32 {
        std::cmp::max(
            Self::max_height_helper(red, blue),
            Self::max_height_helper(blue, red),
        )
    }

    fn max_height_helper(mut x: i32, mut y: i32) -> i32 {
        let mut i = 1;

        loop {
            if i % 2 == 1 {
                x -= i;
                if x < 0 {
                    return i - 1;
                }
            } else {
                y -= i;
                if y < 0 {
                    return i - 1;
                }
            }
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_work() {
        assert_eq!(Solution::max_height_of_triangle(2, 4), 3);
        assert_eq!(Solution::max_height_of_triangle(2, 1), 2);
        assert_eq!(Solution::max_height_of_triangle(1, 1), 1);
        assert_eq!(Solution::max_height_of_triangle(10, 1), 2);
    }
}
