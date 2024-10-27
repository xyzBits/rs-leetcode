use crate::Solution;

struct UnionFind {
    parent: Vec<usize>,
    cnt: usize,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        let parent = (0..n).collect();

        UnionFind { parent, cnt: n }
    }

    fn find(&mut self, i: usize) -> usize {
        if i != self.parent[i] {
            self.parent[i] = self.find(self.parent[i]);
        }

        self.parent[i]
    }

    fn union(&mut self, i: usize, j: usize) -> bool {
        let x = self.find(i);
        let y = self.find(j);

        if x == y {
            true
        } else {
            self.parent[x] = y;
            false
        }
    }
}

impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut union_find = UnionFind::new(edges.len());

        for edge in edges {
            if union_find.union(edge[0] as usize - 1, edge[1] as usize - 1) {
                return edge;
            }
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
            Solution::find_redundant_connection(vec![vec![1, 2], vec![1, 3], vec![2, 3]]),
            vec![2, 3]
        );

        assert_eq!(
            Solution::find_redundant_connection(vec![
                vec![1, 2],
                vec![2, 3],
                vec![3, 4],
                vec![1, 4],
                vec![1, 5]
            ]),
            vec![1, 4]
        );
    }
}
