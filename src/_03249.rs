use crate::Solution;

impl Solution {
    pub fn count_good_nodes(edges: Vec<Vec<i32>>) -> i32 {
        let n = edges.len() + 1;
        let mut g = vec![vec![]; n];
        let mut res = 0;

        for edge in &edges {
            g[edge[0] as usize].push(edge[1]);
            g[edge[1] as usize].push(edge[0]);
        }

        Self::_03249_dfs(0, -1, &g, &mut res);
        res
    }

    fn _03249_dfs(node: usize, parent: i32, g: &Vec<Vec<i32>>, res: &mut i32) -> usize {
        let mut valid = true;
        let mut tree_size = 0;
        let mut sub_tree_size = 0;

        for &child in &g[node] {
            if child != parent {
                let size = Self::_03249_dfs(child as usize, node as i32, g, res);
                if sub_tree_size == 0 {
                    sub_tree_size = size;
                } else if size != sub_tree_size {
                    valid = false;
                }
                tree_size += size;
            }
        }

        if valid {
            *res += 1;
        }
        tree_size + 1
    }
}
