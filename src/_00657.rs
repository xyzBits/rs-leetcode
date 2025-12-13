use crate::Solution;

impl Solution {
    pub fn judge_circle_(moves: String) -> bool {
        let (mut x, mut y) = (0, 0);

        for move_char in moves.chars() {
            if move_char == 'U' {
                y -= 1;
            } else if move_char == 'D' {
                y += 1;
            } else if move_char == 'L' {
                x -= 1;
            } else if move_char == 'R' {
                x += 1;
            }
        }

        x == 0 && y == 0
    }

    pub fn judge_circle(moves: String) -> bool {
        let (mut x, mut y) = (0, 0);

        for move_char in moves.chars() {
            match move_char {
                'U' => x += 1,
                'D' => x -= 1,
                'L' => y += 1,
                'R' => y -= 1,
                _ => {}
            }
        }

        x == 0 && y == 0
    }
}
