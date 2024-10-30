use crate::Solution;

#[derive(Clone)]
struct SegNode {
    v00: i64,
    v01: i64,
    v10: i64,
    v11: i64,
}

impl SegNode {
    fn new() -> Self {
        Self {
            v00: 0,
            v01: 0,
            v10: 0,
            v11: 0,
        }
    }

    fn set(&mut self, v: i64) {
        self.v00 = 0;
        self.v01 = 0;
        self.v10 = 0;
        self.v11 = v.max(0);
    }

    fn best(&self) -> i64 {
        self.v11
    }
}

struct SegTree {
    n: usize,
    tree: Vec<SegNode>,
}

impl SegTree {
    fn new(n: usize) -> Self {
        let tree = vec![SegNode::new(); n * 4 + 1];
        Self { n, tree }
    }

    fn init(&mut self, nums: &[i32]) {
        self.internal_init(nums, 1, 1, self.n);
    }

    fn update(&mut self, x: usize, v: i32) {
        self.internal_update(1, 1, self.n, x + 1, v as i64);
    }

    fn query(&self) -> i64 {
        self.tree[1].best()
    }

    fn internal_init(&mut self, nums: &[i32], x: usize, l: usize, r: usize) {
        if l == r {
            self.tree[x].set(nums[l - 1] as i64);
            return;
        }

        let mid = (l + r) / 2;
        self.internal_init(nums, x * 2, l, mid);
        self.internal_init(nums, x * 2 + 1, mid + 1, r);
        self.push_up(x);
    }

    fn internal_update(&mut self, x: usize, l: usize, r: usize, pos: usize, v: i64) {
        if l > pos || r < pos {
            return;
        }

        if l == r {
            self.tree[x].set(v);
            return;
        }

        let mid = (l + r) / 2;
        self.internal_update(x * 2, l, mid, pos, v);
        self.internal_update(x * 2 + 1, mid + 1, r, pos, v);
        self.push_up(x);
    }
    fn push_up(&mut self, x: usize) {
        let l = x * 2;
        let r = x * 2 + 1;

        self.tree[x].v00 =
            (self.tree[l].v00 + self.tree[r].v10).max(self.tree[l].v01 + self.tree[r].v00);
        self.tree[x].v01 =
            (self.tree[l].v00 + self.tree[r].v11).max(self.tree[l].v01 + self.tree[r].v01);
        self.tree[x].v10 =
            (self.tree[l].v10 + self.tree[r].v10).max(self.tree[l].v11 + self.tree[r].v00);
        self.tree[x].v11 =
            (self.tree[l].v10 + self.tree[r].v11).max(self.tree[l].v11 + self.tree[r].v01);
    }
}

const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn maximum_sum_subsequence(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let n = nums.len();
        let mut tree = SegTree::new(n);
        tree.init(&nums);

        let mut ans = 0;
        for query in queries {
            tree.update(query[0] as usize, query[1]);
            ans = (ans + tree.query()) % MOD;
        }

        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_maximum_sum_subsequence() {
        assert_eq!(
            Solution::maximum_sum_subsequence(vec![3, 5, 9], vec![vec![1, -1], vec![0, -3]]),
            21
        );
        assert_eq!(
            Solution::maximum_sum_subsequence(vec![0, -1], vec![vec![0, -5]]),
            0
        );
    }
}
