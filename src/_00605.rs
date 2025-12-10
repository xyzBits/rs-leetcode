use crate::Solution;

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut count = 0;

        let m = flowerbed.len();
        let mut prev = -1;
        for i in 0..m {
            if flowerbed[i] == 1 {
                if prev < 0 {
                    count += (i as i32) / 2;
                } else {
                    count += ((i as i32) - prev - 2) / 2;
                }

                if count >= n {
                    return true;
                }
                prev = i as i32;
            }
        }

        if prev < 0 {
            count += ((m as i32) + 1) / 2;
        } else {
            count += ((m as i32) - prev - 1) / 2;
        }

        count >= n
    }
}
