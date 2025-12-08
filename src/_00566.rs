use crate::Solution;

impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat[0].len();

        if (m * n) as i32 != r * c {
            return mat;
        }

        let mut ans = vec![vec![0i32; c as usize]; r as usize];
        for x in 0..m * n {
            ans[x / c as usize][x % c as usize] = mat[x / n][x % n];
        }

        ans
    }
}
