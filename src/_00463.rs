use crate::Solution;

impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let dx = vec![0, 1, 0, -1];
        let dy = vec![1, 0, -1, 0];

        let n = grid.len();
        let m = grid[0].len();
        let mut ans = 0;

        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 1 {
                    let mut cnt = 0;
                    for k in 0..4 {
                        let tx = i as i32 + dx[k];
                        let ty = j as i32 + dy[k];
                        if tx < 0
                            || tx >= n as i32
                            || ty < 0
                            || ty >= m as i32
                            || grid[tx as usize][ty as usize] == 0
                        {
                            cnt += 1;
                        }
                    }
                    ans += cnt;
                }
            }
        }

        ans
    }
}
