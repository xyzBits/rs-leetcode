use crate::Solution;

impl Solution {
    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        let n = tops.len();

        let rotations = Self::check_1007(tops[0], &bottoms, &tops, n);

        if rotations != -1 || tops[0] == bottoms[0] {
            rotations
        } else {
            Self::check_1007(bottoms[0], &bottoms, &tops, n)
        }
    }

    fn check_1007(x: i32, a: &Vec<i32>, b: &Vec<i32>, n: usize) -> i32 {
        let mut rotations_a = 0;
        let mut rotations_b = 0;

        for i in 0..n {
            if a[i] != x && b[i] != x {
                return -1;
            } else if a[i] != x {
                rotations_a += 1;
            } else if b[i] != x {
                rotations_b += 1;
            }
        }

        rotations_a.min(rotations_b)
    }
}
