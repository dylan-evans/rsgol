
mod gol;

fn main() {
    let mut grid = gol::Grid::create(20, 20);
    grid.randomise();
    
    for _ in 0..10 {
        let cell = grid.get(1,1);
        if cell {
            println!("Alive");
        } else {
            println!("dead");
        }
        grid.step();
    }
    
}
