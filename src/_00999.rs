use crate::Solution;

impl Solution {
    pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
        let (mut cnt, mut start, mut end) = (0, 0, 0);

        let dx = vec![0, 1, 0, -1];
        let dy = vec![1, 0, -1, 0];
        for i in 0..8 {
            for j in 0..8 {
                if board[i][j] == 'R' {
                    start = i;
                    end = j;
                    break;
                }
            }
        }

        for i in 0..4 {
            let mut step = 0;
            loop {
                let tx = start + step * dx[i] as usize;
                let ty = end + step * dy[i] as usize;
                if tx >= 8 || ty >= 8 || board[tx][ty] == 'B' {
                    break;
                }

                if board[tx][ty] == 'p' {
                    cnt += 1;
                    break;
                }
                step += 1;
            }
        }

        cnt
    }
}
