use crate::Solution;
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut email_to_idx: HashMap<String, Vec<usize>> = HashMap::new();

        for (i, account) in accounts.iter().enumerate() {
            for email in account.iter().skip(1) {
                email_to_idx.entry(email.clone()).or_default().push(i);
            }
        }

        let mut ans = Vec::new();
        let mut visited = vec![false; accounts.len()];


        for i in 0..accounts.len() {
            if visited[i] {
                continue;
            }


            let mut email_set = HashSet::new();
            Self::dfs_helper(i, &accounts, &mut email_to_idx, &mut email_set, &mut visited);
            let mut curr = email_set.into_iter().collect::<Vec<String>>();
            curr.sort();
            curr.insert(0, accounts[i][0].clone());
            ans.push(curr);
        }

        ans
    }

    fn dfs_helper(
        i: usize,
        accounts: &Vec<Vec<String>>,
        email_to_idx: &HashMap<String, Vec<usize>>,
        email_set: &mut HashSet<String>,
        visited: &mut Vec<bool>,
    ) {
        visited[i] = true;

        for email in accounts[i].iter().skip(1) {
            if !email_set.insert(email.clone()) {
                continue;
            }

            if let Some(indices) = email_to_idx.get(email) {
                for &idx in indices {
                    if !visited[idx] {
                        Self::dfs_helper(idx, accounts, email_to_idx, email_set, visited);
                    }
                }
            }
        }
    }
}

