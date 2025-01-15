pub struct GameOfLife {
    width: usize,
    height: usize,
    current_grid: Vec<u64>,
    next_grid: Vec<u64>,
}

impl GameOfLife {
    pub fn new(width: usize, height: usize) -> Self {
        let num_u64s = (width * height + 63) / 64;
        Self {
            width,
            height,
            current_grid: vec![0; num_u64s],
            next_grid: vec![0; num_u64s],
        }
    }

    pub fn set_cell(&mut self, row: usize, col: usize, alive: bool) {
        let index = (row * self.width + col) / 64;
        let bit_pos = (row * self.width + col) % 64;
        if alive {
            self.current_grid[index] |= 1 << bit_pos;
        } else {
            self.current_grid[index] &= !(1 << bit_pos);
        }
    }

    pub fn get_cell(&self, row: usize, col: usize) -> bool {
        let index = (row * self.width + col) / 64;
        let bit_pos = (row * self.width + col) % 64;
        (self.current_grid[index] & (1 << bit_pos)) != 0
    }

    pub fn update(&mut self) {
        for row in 0..self.height {
            for col in 0..self.width {
                let alive_neighbors = self.count_alive_neighbors(row, col);
                let current_alive = self.get_cell(row, col);
                let next_alive = (current_alive && (alive_neighbors == 2 || alive_neighbors == 3)) || (!current_alive && alive_neighbors == 3);
                self.set_cell(row, col, next_alive);
            }
        }
        std::mem::swap(&mut self.current_grid, &mut self.next_grid);
    }

    fn count_alive_neighbors(&self, row: usize, col: usize) -> usize {
        let deltas = [-1, 0, 1];
        let mut count = 0;
        for &dr in &deltas {
            for &dc in &deltas {
                if dr == 0 && dc == 0 {
                    continue;
                }
                let nr = (row as isize + dr + self.height as isize) % self.height as isize;
                let nc = (col as isize + dc + self.width as isize) % self.width as isize;
                if self.get_cell(nr as usize, nc as usize) {
                    count += 1;
                }
            }
        }
        count
    }
}