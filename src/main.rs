extern crate sdl2;

mod gol;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::Point;

fn main() {
    let sdl_ctx = sdl2::init().unwrap();
    let video = sdl_ctx.video().unwrap();
    let mut win = video.window("GOL", 640, 480).position_centered().build().unwrap();
    let mut canvas = win.into_canvas().build().unwrap();
    let mut event_pump = sdl_ctx.event_pump().unwrap();
    canvas.set_draw_color(Color::RGB(200, 255, 255));
    let mut grid = gol::Grid::create(100, 100);
    grid.randomise();
    let mut i = 0;

    'gol_loop: loop {
        let (width, height) = canvas.output_size().unwrap();
        canvas.set_scale(width as f32 / grid.width as f32, height as f32 / grid.height as f32);
        canvas.set_draw_color(Color::RGB(i % 255, 0, 0));
        canvas.clear();
        i = (i + 1) % 254;
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown {..} => {
                    break 'gol_loop;
                },
                _ => {}
            }
        }
        let cell = grid.get(1usize, 1usize);
        if cell {
            println!("Alive");
        } else {
            println!("dead");
        }
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        for x in 0..grid.width {
            for y in 0..grid.height {
                if grid.get(x, y) {
                    canvas.draw_point(Point::new(x as i32, y as i32));
                }
            }
        }
        canvas.present();
        grid.step();
    }
}
