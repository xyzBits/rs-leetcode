use crate::Solution;

impl Solution {
    pub fn super_egg_drop(k: i32, n: i32) -> i32 {
        let mut dp = (0..=n)
            .into_iter()
            .map(|i| i as usize)
            .collect::<Vec<usize>>();

        for j in 2..=k {
            let mut dp2 = vec![0; n as usize + 1];
            let mut x = 1usize;
            for m in 1..=n as usize {
                while x < m
                    && (std::cmp::max(dp[x - 1], dp2[m - x]) > std::cmp::max(dp[x], dp2[m - x - 1]))
                {
                    x += 1;
                }

                dp2[m] = 1 + std::cmp::max(dp[x - 1], dp2[m - x]);
            }

            std::mem::swap(&mut dp, &mut dp2);
        }

        dp[n as usize] as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        // assert_eq!(Solution::super_egg_drop(1, 2), 2);
        assert_eq!(Solution::super_egg_drop(2, 6), 3);
        // assert_eq!(Solution::super_egg_drop(3, 14), 4);
    }
}
