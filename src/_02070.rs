use crate::Solution;

impl Solution {
    pub fn maximum_beauty(mut items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        items.sort_by(|a, b| a[0].cmp(&b[0]));
        let n = items.len();

        for i in 1..n {
            items[i][1] = items[i][1].max(items[i - 1][1]);
        }

        let mut res = vec![0; queries.len()];

        for i in 0..queries.len() {
            res[i] = Self::query(&items, queries[i]);
        }

        res
    }

    fn query(items: &Vec<Vec<i32>>, q: i32) -> i32 {
        let (mut l, mut r) = (0, items.len());

        while l < r {
            let mid = l + (r - l) / 2;
            if items[mid][0] > q {
                r = mid;
            } else {
                l = mid + 1;
            }
        }

        if l == 0 {
            0
        } else {
            items[l - 1][1]
        }
    }
}
