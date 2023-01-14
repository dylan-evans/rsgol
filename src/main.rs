mod gol;
mod renderer;

use clap::Parser;
use crate::renderer::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'W', long, default_value_t = 20)]
    width: usize,
    #[arg(short = 'H', long, default_value_t = 20)]
    height: usize,
    #[arg(short, long, default_value_t = false)]
    fullscreen: bool,
}

fn main() {
    let args = Args::parse();
    let mut renderer = GOLGridRenderer::create(640, 480, args.fullscreen);

    let mut grid = gol::Grid::create(args.width, args.height);
    grid.randomise();

    loop {
        match renderer.get_action() {
            UIAction::Quit => { break },
            UIAction::Reset => {
                grid.randomise();
            },
            _ => {},
        }
        renderer.render_grid(&mut grid);
        grid.step();
    }
}
