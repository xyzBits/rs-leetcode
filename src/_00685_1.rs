use crate::Solution;

struct UnionFind {
    root_set: [usize; 1001],
    mom_set: [usize; 1001],
}

impl UnionFind {
    fn new() -> Self {
        let mut new_set = [0usize; 1001];
        for i in 0..1001 {
            new_set[i] = i;
        }

        Self {
            root_set: new_set.clone(),
            mom_set: new_set,
        }
    }

    fn get_root(&mut self, index: usize) -> usize {
        if self.root_set[index] == index {
            return index;
        }

        // 查找
        let mut root = index;
        while self.root_set[root] != root {
            root = self.root_set[root];
        }

        // 优化
        let mut iter = index;
        while iter != root {
            let temp = self.root_set[iter];
            self.root_set[iter] = root;
            iter = temp;
        }

        root
    }

    // 添加边
    fn add_edge(&mut self, u: usize, v: usize) -> i32 {
        if self.mom_set[v] > u {
            if self.get_root(u) == self.get_root(v) {
                // 添加新的边即是冲突边，又是循环边的情况
                -1
            } else {
                1
            }
        } else {
            if self.get_root(u) == self.get_root(v) {
                2
            } else {
                self.mom_set[v] = u;
                self.root_set[v] = self.mom_set[u];
                0
            }
        }
    }

    fn get_edge(&self, index: usize) -> Vec<i32> {
        vec![self.mom_set[index] as i32, index as i32]
    }
}

impl Solution {
    pub fn find_redundant_directed_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut union_find = UnionFind::new();
        let mut conflict_edges: Option<Vec<i32>> = None;
        let mut loop_edges: Option<Vec<i32>> = None;

        for edge in edges {
            match union_find.add_edge(edge[0] as usize, edge[1] as usize) {
                -1 => return edge,
                1 => conflict_edges = Some(edge),
                2 => loop_edges = Some(edge),
                _ => {}
            }
        }

        match conflict_edges {
            Some(conflict_edge) => match loop_edges {
                Some(loop_edge) => {
                    return union_find.get_edge(conflict_edge[1] as usize);
                }
                None => return conflict_edge,
            },

            None => match loop_edges {
                Some(edge) => return edge,
                None => {}
            },
        }

        vec![]
    }
}
#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_find_redundant_connection() {
        assert_eq!(
            Solution::find_redundant_directed_connection(vec![vec![1, 2], vec![1, 3], vec![2, 3]]),
            vec![2, 3]
        );

        assert_eq!(
            Solution::find_redundant_directed_connection(vec![
                vec![1, 2],
                vec![2, 3],
                vec![3, 4],
                vec![4, 1],
                vec![1, 5]
            ]),
            vec![4, 1]
        );
    }
}
