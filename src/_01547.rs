use crate::Solution;

impl Solution {
    pub fn min_cost(n: i32, mut cuts: Vec<i32>) -> i32 {
        let m = cuts.len();
        cuts.sort();

        let mut new_cuts = vec![0; m + 2];

        for i in 1..=m {
            new_cuts[i] = cuts[i - 1];
        }

        new_cuts[m + 1] = n;

        let mut f = vec![vec![0; m + 2]; m + 2];

        for i in (1..=m).rev() {
            for j in i..=m {
                f[i][j] = if i == j { 0 } else { i32::MAX };

                for k in i..=j {
                    f[i][j] = f[i][j].min(f[i][k - 1] + f[k + 1][j]);
                }

                f[i][j] += new_cuts[j + 1] - new_cuts[i - 1];
            }
        }

        f[1][m]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::min_cost(7, vec![1, 3, 4, 5]), 16);
        assert_eq!(Solution::min_cost(9, vec![5, 6, 1, 4, 2]), 22);
    }
}
