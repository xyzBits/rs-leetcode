use crate::Solution;

impl Solution {
    pub fn moves_to_chessboard(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();

        // 棋盘的第一行与第一列
        let mut row_mask = 0;
        let mut col_mask = 0;
        for i in 0..n {
            row_mask |= board[0][i] << i;
            col_mask |= board[i][0] << i;
        }

        let reverse_row_mask = ((1 << n) - 1) ^ row_mask;
        let reverse_col_mask = ((1 << n) - 1) ^ col_mask;
        let mut row_cnt = 0;
        let mut col_cnt = 0;

        for i in 0..n {
            let mut curr_row_mask = 0;
            let mut curr_col_mask = 0;
            for j in 0..n {
                curr_row_mask |= board[i][j] << j;
                curr_col_mask |= board[j][i] << j;
            }

            // 检测每一行和每一列的状态是否合法
            if (curr_row_mask != row_mask && curr_row_mask != reverse_row_mask)
                || (curr_col_mask != col_mask && curr_col_mask != reverse_col_mask)
            {
                return -1;
            }

            if curr_row_mask == row_mask {
                row_cnt += 1; // 记录与第一行相同的行数
            }
            if curr_col_mask == col_mask {
                col_cnt += 1; // 记录与第一列相同的列数
            }
        }

        fn get_moves(mask: usize, count: i32, n: usize) -> i32 {
            let ones = mask.count_ones() as i32;
            if n % 2 == 1 {
                // 如果 n 为奇数，则每一行中 1 与 0 的数目相差为 1，且满足相邻行交替
                if (n as i32 - 2 * ones).abs() != 1 || (n as i32 - 2 * count).abs() != 1 {
                    return -1;
                }
                if ones == (n / 2) as i32 {
                    // 偶数位变为 1 的最小交换次数
                    (n as i32 / 2) - (mask & 0xAAAAAAAA).count_ones() as i32
                } else {
                    // 奇数位变为 1 的最小交换次数
                    ((n as i32 + 1) / 2) - (mask & 0x55555555).count_ones() as i32
                }
            } else {
                // 如果 n 为偶数，则每一行中 1 与 0 的数目相等，且满足相邻行交替
                if ones != (n / 2) as i32 || count != (n / 2) as i32 {
                    return -1;
                }
                // 偶数位变为 1 的最小交换次数
                let count0 = (n as i32 / 2) - (mask & 0xAAAAAAAA).count_ones() as i32;
                // 奇数位变为 1 的最小交换次数
                let count1 = (n as i32 / 2) - (mask & 0x55555555).count_ones() as i32;
                std::cmp::min(count0, count1)
            }
        }

        let row_moves = get_moves(row_mask as usize, row_cnt, n);
        let col_moves = get_moves(col_mask as usize, col_cnt, n);
        if row_moves == -1 || col_moves == -1 {
            -1
        } else {
            row_moves + col_moves
        }
    }
}
