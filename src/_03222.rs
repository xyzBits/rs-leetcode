use crate::Solution;

impl Solution {
    pub fn losing_player(x: i32, y: i32) -> String {
        let ops = x.min(y / 4);

        if ops % 2 == 1 {
            "Alice".to_string()
        } else {
            "Bob".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_losing_player() {
        assert_eq!(Solution::losing_player(2, 7), "Alice");
        assert_eq!(Solution::losing_player(4, 11), "Bob");
    }
}
