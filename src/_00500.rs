use crate::Solution;

impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        // 1. 定义行索引映射 (rowIdx)
        // 0: bottom row (z, x, c, v, b, n, m)
        // 1: middle row (a, s, d, f, g, h, j, k, l)
        // 2: top row (q, w, e, r, t, y, u, i, o, p)
        // 字符串 "12210111011122000010020202"
        // 对应字母 a-z 的行号
        let row_idx_map: [u8; 26] = [
            1, 2, 2, 1, 0, 1, 1, 1, 0, 1, 1, 1, 2, 2, 0, 0, 0, 0, 1, 0, 0, 2, 0, 2, 0, 2,
        ]; // 0='a', 25='z'

        // 结果列表，对应 Java 的 List<String>
        let mut result_list: Vec<String> = Vec::new();

        // 2. 遍历输入的单词
        for word in words {
            // 跳过空字符串
            if word.is_empty() {
                continue;
            }

            // 将单词转换为小写字节数组，方便索引访问和 char 转换
            let lower_bytes: Vec<u8> = word.to_lowercase().into_bytes();

            // 获取第一个字符的行索引
            // Java: Character.toLowerCase(word.charAt(0)) - 'a'
            // Rust: (lower_bytes[0] - b'a') as usize
            let first_char_index = (lower_bytes[0] - b'a') as usize;
            let target_idx = row_idx_map[first_char_index];

            let mut is_valid = true;

            // 3. 遍历单词的其余字符
            // Java: for (int i = 1; i < word.length(); ++i)
            for i in 1..lower_bytes.len() {
                let current_char_index = (lower_bytes[i] - b'a') as usize;
                let current_idx = row_idx_map[current_char_index];

                // 检查当前字符的行索引是否与第一个字符的行索引相同
                if current_idx != target_idx {
                    is_valid = false;
                    break;
                }
            }

            // 4. 如果有效，将原单词（不是小写版本）加入结果列表
            if is_valid {
                // 将原单词（String）的所有权移动到结果列表中
                result_list.push(word);
            }
        }

        // 5. 返回结果列表 (Rust 的 Vec<String> 直接对应 Java 的 String[])
        result_list
    }
}
