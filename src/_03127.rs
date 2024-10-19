use crate::Solution;

impl Solution {
    pub fn can_make_square(grid: Vec<Vec<char>>) -> bool {
        for i in 0..=1 {
            for j in 0..=1 {
                if Self::check_helper(&grid, i, j) {
                    return true;
                }
            }
        }

        false
    }

    fn check_helper(grid: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
        let mut count = 0;

        for i in 0..=1 {
            for j in 0..=1 {
                if grid[x + i][y + j] == 'B' {
                    count += 1;
                }
            }
        }

        count != 2
    }
}


#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::can_make_square(
            vec![vec!['B', 'W', 'B'], vec!['B', 'W', 'W'], vec!['B', 'W', 'B']]),
                   true
        );


        assert_eq!(Solution::can_make_square(
            vec![vec!['B', 'W', 'B'], vec!['W', 'B', 'W'], vec!['B', 'W', 'B']]),
                   false
        );

        assert_eq!(Solution::can_make_square(
            vec![vec!['B', 'W', 'B'], vec!['B', 'W', 'W'], vec!['B', 'W', 'W']]),
                   true
        );
    }
}
