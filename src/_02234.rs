use crate::Solution;

impl Solution {
    pub fn maximum_beauty(
        flowers: Vec<i32>,
        new_flowers: i64,
        target: i32,
        full: i32,
        partial: i32,
    ) -> i64 {
        let mut flowers = flowers;

        let n = flowers.len();

        for flower in flowers.iter_mut() {
            if *flower > target {
                *flower = target;
            }
        }

        flowers.sort_by(|a, b| b.cmp(a));

        let mut sum = flowers.iter().map(|&flower| flower as i64).sum::<i64>();

        let mut ans = 0;

        if target as i64 * n as i64 - sum <= new_flowers {
            ans = full as i64 * n as i64;
        }

        let mut pre = 0;
        let mut ptr = 0;

        for i in 0..n {
            if i != 0 {
                pre += flowers[i - 1];
            }

            if flowers[i] == target {
                continue;
            }

            let mut rest = new_flowers - ((target * i as i32) as i64 - pre as i64);
            if rest < 0 {
                break;
            }

            while !(ptr >= 1 && flowers[ptr] as i64 * (n - ptr) as i64 - sum <= rest) {
                sum -= flowers[ptr] as i64;
                ptr += 1;
            }

            rest -= flowers[ptr] as i64 * (n - ptr) as i64 - sum;
            ans = std::cmp::max(
                ans,
                full as i64 * i as i64
                    + partial as i64
                        * std::cmp::min(
                            flowers[ptr] as i64 + rest / (n - ptr) as i64,
                            (target - 1) as i64,
                        ),
            );
        }

        ans
    }
}
