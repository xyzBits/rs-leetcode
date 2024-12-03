use crate::Solution;

impl Solution {
    pub fn check_two_chessboards(coordinate1: String, coordinate2: String) -> bool {
        (coordinate1.chars().nth(0).unwrap() as usize
            - coordinate2.chars().nth(0).unwrap() as usize
            + coordinate1.chars().nth(1).unwrap() as usize
            - coordinate2.chars().nth(1).unwrap() as usize)
            % 2
            == 0
    }
}
