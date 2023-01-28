use rand::{thread_rng, Rng};


pub trait Grid {
    fn create(width: usize, height: usize) -> Self where Self: Sized;
    fn get(&self, x: usize, y: usize) -> bool;
    fn set(&mut self, x: usize, y: usize, val: bool);
    fn randomise(&mut self);
    fn flip(&mut self);
    fn step(&mut self);
    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;
}

/// Main GOL area.
pub struct BoolGrid {
    /// Horizontal size
    pub width: usize,
    /// Vertical size
    pub height: usize,
    /// An array of 2 vectors containing the living state of each cell within the `Grid`
    locations: [Vec<bool>; 2],
    /// Indicates the current "read" vector in locations.
    current_location: usize,
}

impl BoolGrid {
    /// Returns the index of the next location vector
    fn next_location(&self) -> usize {
        (self.current_location + 1) % 2
    }

    /// Convert X,Y coordinates into a vector index
    fn get_offset(&self, x: usize, y: usize) -> usize {
        if x > self.width || y > self.height {
            panic!("Value out of range");
        }
        return (y * self.width) + x;
    }

    /// Count the number of adjacent living cells
    fn count_neighbours(&self, x: usize, y: usize) -> usize {
        let mut count: usize = 0;
        let range: [i64; 3] = [-1, 0, 1];
        for x_mod in range {
            let x_ref: i64 = x as i64 + x_mod;
            for y_mod in range {
                let y_ref = y as i64 + y_mod;
                if (x_mod == 0 && y_mod == 0)
                    || (x_ref as usize == x && y_ref as usize == y)
                    || x_ref < 0
                    || x_ref as usize >= self.width
                    || y_ref < 0
                    || y_ref as usize >= self.height
                {
                    continue;
                }

                if self.get(x_ref as usize, y_ref as usize) {
                    count += 1;
                }
            }
        }
        return count;
    }

    /// Apply the GOL rules to the specified cell and return the next state.
    fn calculate_next_cell_state(&self, x: usize, y: usize) -> bool {
        let neighbours = self.count_neighbours(x, y);
        neighbours == 3 || (self.get(x, y) && neighbours == 2)
    }
}

impl Grid for BoolGrid {
    fn create(width: usize, height: usize) -> Self where Self: Sized {
        let mut locs: Vec<bool> = Vec::with_capacity(width * height);
        locs.resize(width * height, false);
        BoolGrid {
            width,
            height,
            current_location: 0,
            locations: [locs.clone(), locs],
        }
    }

    /// Get the state of the specified cell in the current location vector.
    fn get(&self, x: usize, y: usize) -> bool {
        self.locations[self.current_location][self.get_offset(x, y)]
    }

    /// Set the state of the specified cell in the next location vector.
    fn set(&mut self, x: usize, y: usize, val: bool) {
        let ofs = self.get_offset(x, y);
        self.locations[self.next_location()][ofs] = val;
    }

    fn randomise(&mut self) {
        let mut rng = thread_rng();
        for idx in 0..(self.width * self.height) {
            self.locations[self.current_location][idx] = rng.gen_bool(0.5);
        }
    }

    /// Swaps the location vectors so the next becomes current.
    fn flip(&mut self) {
        self.current_location = self.next_location();
    }

    /// Perform a GOL step, calculating all values in the next location and
    /// flipping the locations.
    fn step(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.set(x, y, self.calculate_next_cell_state(x, y));
            }
        }
        self.flip();
    }

    fn get_width(&self) -> usize {
        self.width
    }

    fn get_height(&self) -> usize {
        self.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_blinker() -> Grid {
        let mut grid = BoolGrid::create(3, 3);
        grid.set(1, 0, true);
        grid.set(1, 1, true);
        grid.set(1, 2, true);
        grid.flip();
        return grid;
    }

    #[test]
    fn simple_blinker() {
        let grid = setup_blinker();
        assert!(grid.get(1, 0));
        assert!(grid.get(1, 1));
        assert!(grid.get(1, 2));
    }

    #[test]
    fn gol_blinker() {
        let mut grid = setup_blinker();
        grid.step();
        assert!(!grid.get(1, 0));
        assert!(!grid.get(1, 2));
        assert!(grid.get(0, 1));
        assert!(grid.get(1, 1));
        assert!(grid.get(2, 1));
    }
}
