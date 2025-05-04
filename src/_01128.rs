use crate::Solution;

impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        let mut num = vec![0; 100];
        let mut ans = 0;

        for domino in dominoes {
            let val = if domino[0] < domino[1] {
                domino[0] * 10 + domino[1]
            } else {
                domino[1] * 10 + domino[0]
            };

            ans += num[val as usize];
            num[val as usize] += 1
        }

        ans
    }
}
