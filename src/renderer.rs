extern crate sdl2;

use crate::gol::Grid;

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

pub struct GOLGridRenderer {
    canvas: Canvas<Window>,
    event_pump: EventPump,
}

impl GOLGridRenderer {
    pub fn create(width: u32, height: u32) -> Self {
        let sdl_ctx = sdl2::init().unwrap();
        let video = sdl_ctx.video().unwrap();
        let window = video
            .window("GOL", width, height)
            .resizable()
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();
        let event_pump = sdl_ctx.event_pump().unwrap();
        return GOLGridRenderer { canvas, event_pump };
    }

    pub fn render_grid(&mut self, grid: &Grid) {
        self.canvas.set_draw_color(Color::RGB(64, 0, 0));
        let (width, height) = self.canvas.output_size().unwrap();
        self.canvas
            .set_scale(
                width as f32 / grid.width as f32,
                height as f32 / grid.height as f32,
            )
            .unwrap();
        self.canvas.clear();
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        for x in 0..grid.width {
            for y in 0..grid.height {
                if grid.get(x, y) {
                    self.canvas
                        .draw_point(Point::new(x as i32, y as i32))
                        .unwrap();
                }
            }
        }
        self.canvas.present();
    }

    pub fn quit(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { .. } => {
                    return true;
                }
                _ => {}
            }
        }

        return false;
    }
}
