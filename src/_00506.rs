use crate::Solution;

impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let n = score.len();

        // 1. 定义奖牌描述 (对应 Java 的 String[] desc)
        let desc = ["Gold Medal", "Silver Medal", "Bronze Medal"];

        // 2. 将分数与原始索引绑定 (对应 Java 的 int[][] arr)
        // 创建一个 Vec<(i32, usize)>，存储 (分数, 原始索引)
        let mut arr: Vec<(i32, usize)> = score
            .into_iter()
            .enumerate() // 生成 (索引, 分数)
            .map(|(index, s)| (s, index)) // 调整为 (分数, 索引)
            .collect();

        // 3. 按分数降序排序 (对应 Java 的 Arrays.sort, b[0] - a[0])
        // sort_unstable_by 是高效且常用的排序方法
        arr.sort_unstable_by(|a, b| b.0.cmp(&a.0)); // b.0 是分数，进行降序比较

        // 4. 初始化结果数组 (对应 Java 的 String[] ans)
        let mut ans: Vec<String> = vec![String::new(); n];

        // 5. 遍历排序后的数组，确定排名并填充结果
        // Java: for (int i = 0; i < n; ++i)
        for (rank_index, (score, original_index)) in arr.into_iter().enumerate() {
            // rank_index 是 0-based 排名

            // 排名 (1-based)
            let rank = rank_index + 1;

            let rank_str = if rank_index < 3 {
                // 前三名：使用奖牌描述
                // Java: desc[i]
                desc[rank_index].to_string()
            } else {
                // 其它排名：使用数字字符串
                // Java: Integer.toString(i + 1)
                rank.to_string()
            };

            // 将排名字符串存回原始索引对应的位置
            // Java: ans[arr[i][1]] = ...
            ans[original_index] = rank_str;
        }

        ans
    }
}
