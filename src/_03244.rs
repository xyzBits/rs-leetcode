use crate::Solution;

impl Solution {
    pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut roads = (1..=n).collect::<Vec<_>>();

        let mut ans = vec![0; queries.len()];
        let mut dist = n - 1;

        for i in 0..queries.len() {
            let mut k = roads[queries[i][0] as usize];
            roads[queries[i][0] as usize] = queries[i][1];
            while k != -1 && k < queries[i][1] {
                let t = roads[k as usize];
                roads[k as usize] = -1;
                k = t;
                dist -= 1;
            }
            ans[i] = dist;
        }

        ans
    }
}
