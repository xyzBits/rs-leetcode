use crate::Solution;
use std::collections::HashSet;

struct UnionFind {
    union: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> UnionFind {
        UnionFind {
            union: (0..size).collect(),
        }
    }

    fn find(&mut self, mut a: usize) -> usize {
        while a != self.union[a] {
            a = self.union[a];
        }

        a
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let union_a = self.find(a);
        let union_b = self.find(b);
        if union_a == union_b {
            true
        } else {
            self.union[union_b] = union_a;
            false
        }
    }
}

impl Solution {
    pub fn find_redundant_directed_connection_(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();
        let mut co: HashSet<i32> = (1..n as i32 + 1).collect();
        for i in edges.iter() {
            co.remove(&i[1]);
        }

        let mut root = 0;
        for i in co {
            root = i;
        }

        let mut uf = UnionFind::new(n);
        if root == 0 {
            for i in edges.into_iter() {
                if uf.union(i[0] as usize - 1, i[1] as usize - 1) {
                    return i;
                }
            }
            return vec![-1];
        }

        let root = root as usize - 1;
        let mut co = vec![n; n];
        let mut ru1 = 0;
        let mut ru2 = 0;
        let mut ru = 0;

        for i in 0..edges.len() {
            if co[edges[i][1] as usize - 1] != n {
                ru = edges[i][1] as usize - 1;
                ru1 = co[edges[i][1] as usize - 1];
                ru2 = edges[i][0] as usize - 1;
                break;
            }
            co[edges[i][1] as usize - 1] = edges[i][0] as usize - 1;
        }

        for i in edges.iter() {
            if i[1] as usize - 1 == ru {
                continue;
            }
            uf.union(i[0] as usize - 1, i[1] as usize - 1);
        }

        if uf.find(ru1) == root {
            return vec![ru2 as i32 + 1, ru as i32 + 1];
        }

        if uf.find(ru2) == root {
            return vec![ru1 as i32 + 1, ru as i32 + 1];
        }

        vec![root as i32]
    }
}
#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_find_redundant_connection() {
        assert_eq!(
            Solution::find_redundant_directed_connection_(vec![vec![1, 2], vec![1, 3], vec![2, 3]]),
            vec![2, 3]
        );

        assert_eq!(
            Solution::find_redundant_directed_connection_(vec![
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
