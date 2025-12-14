use crate::Solution;

impl Solution {
    pub fn shortest_distance_after_queries_(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut prev = vec![vec![]; n as usize];

        let mut dp = vec![0; n as usize];
        for i in 1..n as usize {
            prev.get_mut(i).unwrap().push(i as i32 - 1);
            dp[i] = i as i32;
        }

        let mut ans = vec![0; queries.len()];

        for i in 0..queries.len() {
            prev.get_mut(queries[i][1] as usize)
                .unwrap()
                .push(queries[i][0]);
            for v in queries[i][1]..n {
                for &u in prev.get(v as usize).unwrap() {
                    dp[v as usize] = dp[v as usize].min(dp[u as usize] + 1);
                }
            }
            ans[i] = dp[n as usize - 1];
        }

        ans
    }
}
