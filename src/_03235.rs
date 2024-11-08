use crate::Solution;

impl Solution {
    pub fn can_reach_corner(x_corner: i32, y_corner: i32, circles: Vec<Vec<i32>>) -> bool {
        let mut visited = vec![false; circles.len()];

        for i in 0..circles.len() {
            let circle = &circles[i];
            let x = circle[0];
            let y = circle[1];
            let r = circle[2];

            if Self::point_in_circle(0, 0, x, y, r)
                || Self::point_in_circle(x_corner, y_corner, x, y, r)
            {
                return false;
            }

            if !visited[i]
                && Self::circle_intersects_top_left_of_rectangle(x, y, r, x_corner, y_corner)
                && Self::dfs_3235(i, &circles, x_corner, y_corner, &mut visited)
            {
                return false;
            }
        }

        true
    }

    fn dfs_3235(
        i: usize,
        circles: &Vec<Vec<i32>>,
        x_corner: i32,
        y_corner: i32,
        visited: &mut Vec<bool>,
    ) -> bool {
        let x1 = circles[i][0];
        let y1 = circles[i][1];
        let r1 = circles[i][2];

        if Self::circle_intersects_bottom_right_of_rectangle(x1, y1, r1, x_corner, y_corner) {
            return true;
        }

        visited[i] = true;

        for j in 0..circles.len() {
            if !visited[j] {
                let x2 = circles[j][0];
                let y2 = circles[j][1];
                let r2 = circles[j][2];
                if Self::circles_intersect_in_rectangle(x1, y1, r1, x2, y2, r2, x_corner, y_corner)
                    && Self::dfs_3235(j, circles, x_corner, y_corner, visited)
                {
                    return true;
                }
            }
        }

        false
    }

    fn point_in_circle(px: i32, py: i32, x: i32, y: i32, r: i32) -> bool {
        ((x - px) as i128).pow(2) + ((y - py) as i128).pow(2) <= (r as i128).pow(2)
    }

    fn circle_intersects_top_left_of_rectangle(
        x: i32,
        y: i32,
        r: i32,
        x_corner: i32,
        y_corner: i32,
    ) -> bool {
        (x.abs() <= r && y >= 0 && y <= y_corner)
            || (x >= 0 && x <= x_corner && (y - y_corner).abs() <= r)
            || Self::point_in_circle(0, y_corner, x, y, r)
    }

    fn circle_intersects_bottom_right_of_rectangle(
        x: i32,
        y: i32,
        r: i32,
        x_corner: i32,
        y_corner: i32,
    ) -> bool {
        (y.abs() <= r && 0 <= x && x <= x_corner)
            || (0 <= y && y <= y_corner && (x - x_corner).abs() <= r)
            || Self::point_in_circle(x_corner, 0, x, y, r)
    }

    fn circles_intersect_in_rectangle(
        x1: i32,
        y1: i32,
        r1: i32,
        x2: i32,
        y2: i32,
        r2: i32,
        x_corner: i32,
        y_corner: i32,
    ) -> bool {
        ((x1 - x2) as i128).pow(2) + ((y1 - y2) as i128).pow(2) <= ((r1 + r2) as i128).pow(2)
            && ((x1 * r2) as i128 + (x2 * r1) as i128) < ((r1 + r2) as i128 * x_corner as i128)
            && ((y1 * r2) as i128 + (y2 * r1) as i128) < (r1 + r2) as i128 * y_corner as i128
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test1() {
        assert_eq!(Solution::can_reach_corner(3, 4, vec![vec![2, 1, 1]]), true);

        assert_eq!(Solution::can_reach_corner(3, 3, vec![vec![1, 1, 2]]), false);

        assert_eq!(
            Solution::can_reach_corner(3, 3, vec![vec![2, 1, 1], vec![1, 2, 1]]),
            false
        );

        assert_eq!(Solution::can_reach_corner(4, 4, vec![vec![5, 5, 1]]), true);

        assert_eq!(
            Solution::can_reach_corner(15, 15, vec![vec![1, 99, 85], vec![99, 1, 85]]),
            true
        );
    }
}
