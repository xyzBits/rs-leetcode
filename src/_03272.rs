use crate::Solution;
use std::collections::HashSet;

impl Solution {
    pub fn count_good_integers(n: i32, k: i32) -> i64 {
        let mut dict: HashSet<String> = HashSet::new();
        let mut base = 10_i32.pow(((n - 1) / 2) as u32);

        let skip = (n & 1) as usize;

        for i in base..base * 10 {
            let s = i.to_string();
            let rev = s.chars().rev().skip(skip).collect::<String>();
            let combined = format!("{}{}", s, rev);
            let palindromic_integer = combined.parse::<i64>().unwrap();

            if palindromic_integer % (k as i64) == 0 {
                let mut sorted_chars = combined.chars().collect::<Vec<char>>();
                sorted_chars.sort();
                dict.insert(sorted_chars.into_iter().collect());
            }
        }

        let mut factorial = vec![1; (n + 1) as usize];
        for i in 1..=n as usize {
            factorial[i] = factorial[i - 1] * i as i64;
        }

        let mut ans = 0;

        for s in dict {
            let mut cnt = vec![0; 10];
            for c in s.chars() {
                cnt[c.to_digit(10).unwrap() as usize] += 1;
            }

            let mut tot = (n as i64 - cnt[0] as i64) * factorial[(n - 1) as usize];
            for &x in &cnt {
                tot /= factorial[x];
            }

            ans += tot;
        }

        ans
    }
}
