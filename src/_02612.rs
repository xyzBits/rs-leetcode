use crate::Solution;
use std::collections::{BTreeSet, HashSet, VecDeque};

impl Solution {
    pub fn min_reverse_operations(n: i32, p: i32, banned: Vec<i32>, k: i32) -> Vec<i32> {
        let n = n as usize;
        let p = p as usize;
        let k = k as usize;

        let ban: HashSet<i32> = banned.into_iter().collect();
        let mut sets = vec![BTreeSet::new(), BTreeSet::new()];
        for i in 0..n {
            if i != p && !ban.contains(&(i as i32)) {
                sets[i % 2].insert(i as i32);
            }
        }

        let mut ans = vec![-1; n];
        let mut q = VecDeque::new();

        q.push_back(p as i32);
        ans[p] = 0;

        while let Some(i) = q.pop_front() {
            let i = i as usize;
            let mn = (i as i32 - k as i32 + 1).max(k as i32 - i as i32 - 1) as usize;
            let mx = (i as i32 + k as i32 - 1).min(n as i32 * 2 - k as i32 - i as i32 - 1) as usize;
            let set = &mut sets[mx % 2];
            let mut to_remove = Vec::new();

            for &val in set.range(mn as i32..=mx as i32) {
                ans[val as usize] = ans[i] + 1;
                q.push_back(val);
                to_remove.push(val);
            }

            for val in to_remove {
                set.remove(&val);
            }
        }
        ans
    }
}

#[test]
fn test_01() {
    let mut set = BTreeSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(4);
    set.insert(5);
    set.insert(6);

    for num in set.range(2..=4) {
        println!("{}", num);
    }

    let mut vector = vec![1, 2, 3, 4, 5, 6];

    for num in &vector[2..4] {
        println!("{}", num);
    }
}
