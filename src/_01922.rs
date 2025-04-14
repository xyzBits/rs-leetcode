use crate::Solution;

const MOD: i64 = 1000000007;

impl Solution {
    pub fn count_good_numbers(n: i64) -> i32 {
        (Self::quick_mul(5, (n + 1) / 2) * Self::quick_mul(4, n / 2) % MOD) as i32
    }

    // 快速幂求出 x^y % mod
    fn quick_mul(x: i32, mut y: i64) -> i64 {
        let mut ret = 1i64;
        let mut mul = x as i64;
        while y > 0 {
            if y % 2 == 1 {
                ret = ret * mul % MOD;
            }
            mul = mul * mul % MOD;
            y /= 2;
        }
        ret
    }
}
