use crate::Solution;

impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let n = score.len();
        let desc = vec!["Gold Medal", "Silver Medal", "Bronze Medal"];
        let mut arr: Vec<(i32, usize)> = score
            .into_iter()
            .enumerate()
            .map(|(index, s)| (s, index))
            .collect();

        arr.sort_unstable_by(|a, b| b.0.cmp(&a.0));

        let mut ans = Vec::with_capacity(n);

        for (rank_index, (score, origin_index)) in arr.into_iter().enumerate() {
            let rank = rank_index + 1;

            let rank_str = if rank_index < 3 {
                desc[rank_index].to_string()
            } else {
                rank.to_string()
            };

            ans[origin_index] = rank_str;
        }

        ans
    }
}
