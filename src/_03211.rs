use crate::Solution;

impl Solution {
    pub fn valid_strings(n: i32) -> Vec<String> {
        let mut res = Vec::new();
        let mut arr = Vec::new();

        Self::inner_dfs_3211(&mut arr, n, &mut res);

        res
    }

    fn inner_dfs_3211(arr: &mut Vec<char>, n: i32, res: &mut Vec<String>) {
        if arr.len() == n as usize {
            res.push(arr.iter().collect());
            return;
        }

        if arr.is_empty() || arr[arr.len() - 1] == '1' {
            arr.push('0');
            Self::inner_dfs_3211(arr, n, res);
            arr.pop();
        }

        arr.push('1');
        Self::inner_dfs_3211(arr, n, res);
        arr.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::valid_strings(3),
            vec![
                "010".to_string(),
                "011".to_string(),
                "101".to_string(),
                "110".to_string(),
                "111".to_string()
            ]
        );
        assert_eq!(
            Solution::valid_strings(1),
            vec!["0".to_string(), "1".to_string()]
        );
    }
}
