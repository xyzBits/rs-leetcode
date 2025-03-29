use crate::Solution;

impl Solution {
    pub fn longest_cycle(edges: Vec<i32>) -> i32 {
        let n = edges.len();
        let mut label = vec![0; n];
        let mut current_label = 0;
        let mut ans = -1;

        for i in 0..n {
            if label[i] != 0 {
                continue;
            }

            let mut pos = i as i32;
            let start_label = current_label;
            while pos != -1 {
                current_label += 1;

                if label[pos as usize] != 0 {
                    if label[pos as usize] > start_label {
                        ans = ans.max(current_label - label[pos as usize]);
                    }
                    break;
                }
                label[pos as usize] = current_label;
                pos = edges[pos as usize];
            }
        }

        ans
    }
}
