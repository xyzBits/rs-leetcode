use crate::Solution;

impl Solution {
    pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = img.len();
        let n = img[0].len();

        let mut ans = vec![vec![0; n]; m];

        for i in 0..m {
            for j in 0..n {
                let (mut num, mut sum) = (0, 0);
                // for x in i - 1..=(i + 1) {
                //     for y in j - 1..=(j + 1) {
                for x in i.saturating_sub(1)..=(i + 1).min(m - 1) {
                    for y in j.saturating_sub(1)..=(j + 1).min(n - 1) {
                        if x >= 0 && x < m && y >= 0 && y < n {
                            num += 1;
                            sum += img[x][y];
                        }
                    }
                }

                ans[i][j] = sum / num;
            }
        }

        ans
    }
}
