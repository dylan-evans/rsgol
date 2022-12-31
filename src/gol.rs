
use rand::{thread_rng, Rng};


pub struct Grid {
    width: u64,
    height: u64,
    current_location: usize,
    locations: [Vec<bool>; 2],
}


impl Grid {
    pub fn create(width: u64, height: u64) -> Self {
        let mut locs: Vec<bool> = Vec::with_capacity((width * height) as usize);
        locs.resize((width * height) as usize, false);
        Grid {
            width,
            height,
            current_location: 0,
            locations: [locs.clone(), locs],
        }
    }

    fn next_location(&self) -> usize {
        (self.current_location + 1) % 1
    }

    fn flip(&mut self) {
        self.current_location = self.next_location();
    }

    fn get_offset(&self, x: u64, y: u64) -> usize {
        if x > self.width || y > self.height {
            panic!("Value out of range");
        }
        return ((y * self.width) + x) as usize;
    }

    fn count_neighbours(&self, x: u64, y: u64) -> u64 {
        let mut count: u64 = 0;
        let range: [i64; 3] = [-1, 0, 1];
        for x_mod in range  {
            let x_ref: i64 = x as i64 + x_mod;
            for y_mod in range {
                let y_ref = y as i64 + y_mod;
                if (x_mod == 0 && y_mod == 0) || x_ref < 0 || y_ref < 0 ||
                    y_ref as u64 >= self.height || x_ref as u64 >= self.width {
                    continue;
                }

                if self.get(x_ref as u64, y_ref as u64) {
                    count += 1;
                }
            }
        }
        return count;
    }

    fn get_next_value(&self, x: u64, y: u64) -> bool {
        let neighbours = self.count_neighbours(x, y);
        (self.get(x, y) && neighbours == 2) || neighbours == 3
    }

    pub fn get(&self, x: u64, y: u64) -> bool {
        self.locations[self.current_location][self.get_offset(x, y)]
    }

    pub fn set(&mut self, x: u64, y: u64, val: bool) {
        let ofs = self.get_offset(x, y);
        self.locations[self.next_location()][ofs] = val;
    }

    pub fn randomise(&mut self) {
        let mut rng = thread_rng();
        for idx in 0..(self.width * self.height) {
            self.locations[self.current_location][idx as usize] = rng.gen_bool(0.5);
        }
    }

    pub fn step(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.set(x, y, self.get_next_value(x, y));
            }
        }
        self.flip();
    }

}

