use crate::Solution;

impl Solution {
    pub fn find_permutation(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut f = vec![vec![0; n]; 1 << n];
        let mut g = vec![vec![i32::MAX; n]; 1 << n];

        for j in 0..n {
            f[(1 << n) - 1][j] = (j as i32 - nums[0]).abs();
        }

        g[(1 << n) - 1] = vec![-1; n];

        for s in (1..((1 << n) - 4)).step_by(2).rev() {
            for j in 0..n {
                if (s >> j & 1) == 0 {
                    continue;
                }

                for k in 1..n {
                    if (s >> k & 1) == 0 {
                        continue;
                    }

                    let v = f[s | 1 << k][k] + (j as i32 - nums[k]).abs();
                    if v < f[s][j] {
                        f[s][j] = v;
                        g[s][j] = k as i32;
                    }
                }
            }
        }

        let mut ans = vec![0; n];
        let (mut s, mut i, mut j) = (0, 0, 0);
        while j != !0 {
            // !0 is usize::MAX, equivalent to -1 as usize
            ans[i] = j as i32;
            i += 1;
            s |= 1 << (j as i32);
            let next_val = g[s as usize][j];
            if next_val == -1 {
                break;
            }
            j = next_val as usize;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_find_permutation() {
        assert_eq!(Solution::find_permutation(vec![1, 0, 2]), vec![0, 1, 2]);
        assert_eq!(Solution::find_permutation(vec![0, 2, 1]), vec![0, 2, 1]);
    }
}
