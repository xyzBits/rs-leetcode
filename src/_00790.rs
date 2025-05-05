use crate::Solution;

impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        let mut f = vec![1, 0, 0, 0];
        let mod_num = 1_000_000_000 + 7;

        for _ in 1..=n {
            let mut g = vec![0_i64; 4];
            g[0] = (f[0] + f[1] + f[2] + f[3]) % mod_num;
            g[1] = (f[2] + f[3]) % mod_num;
            g[2] = (f[1] + f[3]) % mod_num;
            g[3] = f[0];
            f = g;
        }

        f[0] as i32
    }
}
