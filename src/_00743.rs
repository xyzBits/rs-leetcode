use crate::Solution;

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        const INF: i32 = i32::MAX / 2;

        let mut g = vec![vec![INF; n as usize]; n as usize];

        for time in &times {
            let x = time[0] as usize - 1;
            let y = time[1] as usize - 1;
            g[x][y] = time[2];
        }

        let mut dist = vec![INF; n as usize];
        dist[k as usize - 1] = 0;
        let mut used = vec![false; n as usize];

        for _i in 0..n as usize {
            let mut x = -1_i32;
            for y in 0..n as usize {
                if !used[y] && (x == -1 || dist[y] < dist[x as usize]) {
                    x = y as i32;
                }
            }
            used[x as usize] = true;

            for y in 0..n as usize {
                dist[y] = dist[y].min(dist[x as usize] + g[x as usize][y]);
            }
        }

        let ans = *dist.iter().max().unwrap();
        if ans == INF {
            -1
        } else {
            ans
        }
    }
}
