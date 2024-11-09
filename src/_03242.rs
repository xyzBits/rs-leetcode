use std::collections::HashMap;

const DIRS: [&[(i32, i32)]; 2] = [
    &[(-1, 0), (1, 0), (0, -1), (0, 1)],
    &[(-1, -1), (-1, 1), (1, -1), (1, 1)],
];

struct NeighborSum {
    grid: Vec<Vec<i32>>,
    pos: HashMap<i32, (usize, usize)>,
}

impl NeighborSum {
    fn new(grid: Vec<Vec<i32>>) -> Self {
        let mut pos = HashMap::new();
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {

                // grid[i][j] 可以 copy
                pos.insert(grid[i][j], (i, j));
            }
        }

        NeighborSum { grid, pos }
    }

    fn adjacent_sum(&self, value: i32) -> i32 {
        self.get_sum(value, 0)
    }

    fn diagonal_sum(&self, value: i32) -> i32 {
        self.get_sum(value, 1)
    }

    fn get_sum(&self, value: i32, idx: usize) -> i32 {
        if let Some(&(x, y)) = self.pos.get(&value) {
            let mut sum = 0;
            for &(dx, dy) in DIRS[idx] {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && (nx as usize) < self.grid.len() && ny >= 0 && (ny as usize) < self.grid[0].len() {
                    sum += self.grid[nx as usize][ny as usize];
                }
            }
            sum
        } else {
            0
        }
    }
}