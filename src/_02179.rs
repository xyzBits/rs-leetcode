use crate::Solution;

struct FenwickTree {
    tree: Vec<i32>,
}

impl FenwickTree {
    fn new(size: usize) -> Self {
        Self {
            tree: vec![0; size + 1],
        }
    }

    fn update(&mut self, mut index: usize, delta: i32) {
        index += 1;
        while index < self.tree.len() {
            self.tree[index] += delta;
            index += index & (!index + 1);
        }
    }

    fn query(&self, mut index: usize) -> i32 {
        index += 1;
        let mut res = 0;

        while index > 0 {
            res += self.tree[index];
            index -= index & (!index + 1);
        }

        res
    }
}
impl Solution {
    pub fn good_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let n = nums1.len();
        let mut pos2 = vec![0; n];
        let mut reversed_index_mapping = vec![0; n];

        for i in 0..n {
            pos2[nums2[i] as usize] = i;
        }

        for i in 0..n {
            reversed_index_mapping[pos2[nums1[i] as usize]] = i;
        }

        let mut tree = FenwickTree::new(n);
        let mut res = 0_i64;

        for value in 0..n {
            let pos = reversed_index_mapping[value];
            let left = tree.query(pos) as i64;
            tree.update(pos, 1);
            let right = (n - 1 - pos) as i64 - (value as i64 - left);
            res += left * right;
        }

        res
    }
}
