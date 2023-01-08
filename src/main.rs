
mod gol;
mod renderer;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'W', long, default_value_t = 20)]
    width: usize,
    #[arg(short = 'H', long, default_value_t = 20)]
    height: usize,
}

fn main() {
    let args = Args::parse();
    let mut renderer = renderer::GOLGridRenderer::create(640, 480);

    let mut grid = gol::Grid::create(args.width, args.height);
    grid.randomise();

    loop {
        if renderer.quit() {
            break;
        }
        renderer.render_grid(&mut grid);
        grid.step();
    }
}
