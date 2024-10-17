use crate::Solution;

use std::collections::HashMap;
const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn number_of_permutations(n: i32, requirements: Vec<Vec<i32>>) -> i32 {
        let mut req_map = HashMap::new();
        req_map.insert(0, 0);
        let mut max_cnt = 0;
        for i in 0..requirements.len() {
            let end = requirements[i][0];
            let cnt = requirements[i][1];
            req_map.insert(end, cnt);
            if cnt > max_cnt {
                max_cnt = cnt;
            }
        }
        if *req_map.get(&0).unwrap() != 0 {
            return 0;
        }
        let mut dp = vec![vec![-1; max_cnt as usize + 1]; n as usize];

        Self::inner_dfs(
            n as usize - 1,
            *req_map.get(&(n - 1)).unwrap() as usize,
            &req_map,
            &mut dp,
        ) as i32
    }

    fn inner_dfs(
        end: usize,
        cnt: usize,
        req_map: &HashMap<i32, i32>,
        dp: &mut Vec<Vec<i64>>,
    ) -> i64 {
        if end == 0 {
            return 1;
        }
        if dp[end][cnt] != -1 {
            return dp[end][cnt];
        }
        if let Some(&r) = req_map.get(&(end as i32 - 1)) {
            if r as usize <= cnt && cnt <= end + r as usize {
                dp[end][cnt] = Self::inner_dfs(end - 1, r as usize, req_map, dp);
                return dp[end][cnt];
            }
            return 0;
        }
        let mut tot_sum = 0;
        for i in 0..=cnt.min(end) {
            tot_sum = (tot_sum + Self::inner_dfs(end - 1, cnt - i, req_map, dp)) % MOD;
        }
        dp[end][cnt] = tot_sum;
        tot_sum
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::number_of_permutations(3, vec![vec![2, 2], vec![0, 0]]),
            2
        );
        assert_eq!(
            Solution::number_of_permutations(3, vec![vec![2, 2], vec![1, 1], vec![0, 0]]),
            1
        );
        assert_eq!(
            Solution::number_of_permutations(2, vec![vec![0, 0], vec![1, 0]]),
            1
        );
    }
}
