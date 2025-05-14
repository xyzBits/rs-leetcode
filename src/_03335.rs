use crate::Solution;

impl Solution {
    pub fn length_after_transformations_(s: String, t: i32) -> i32 {
        const MOD: i64 = 1000000007;
        let mut cnt = vec![0; 26];
        for &ch in s.as_bytes() {
            cnt[ch as usize - 'a' as usize] += 1;
        }

        for round in 0..t {
            let mut nxt = vec![0; 26];
            nxt[0] = cnt[25];
            nxt[1] = (cnt[25] + cnt[0]) % MOD;
            for i in 2..26 {
                nxt[i] = cnt[i - 1];
            }
            cnt = nxt;
        }

        let mut ans = 0;
        for i in 0..26 {
            ans = (ans + cnt[i]) % MOD;
        }

        ans as i32
    }
}

#[test]
fn test_find_even_numbers() {
    assert_eq!(1, 1);
}
