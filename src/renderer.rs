extern crate sdl2;

use crate::gol::Grid;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::{FullscreenType, Window};
use sdl2::EventPump;

pub enum UIAction {
    Nothing,
    Quit,
    Reset,
}

pub struct GOLGridRenderer {
    canvas: Canvas<Window>,
    event_pump: EventPump,
}

impl GOLGridRenderer {
    pub fn create(width: u32, height: u32, fullscreen: bool) -> Self {
        let sdl_ctx = sdl2::init().unwrap();
        let video = sdl_ctx.video().unwrap();
        let mut window = video
            .window("GOL", width, height)
            .opengl()
            .resizable()
            .position_centered()
            .build()
            .unwrap();
        if fullscreen {
            window.set_fullscreen(FullscreenType::Desktop).unwrap();
        }
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

    pub fn toggle_fullscreen(&mut self) {
        let window = self.canvas.window_mut();
        if window.fullscreen_state() == FullscreenType::Off {
            window.set_fullscreen(FullscreenType::Desktop).unwrap();
        } else {
            window.set_fullscreen(FullscreenType::Off).unwrap();
        }
    }

    pub fn get_action(&mut self) -> UIAction {
        loop {
            match self.event_pump.poll_event() {
                Some(event) => {
                    match event {
                        Event::KeyDown {keycode: Some(keycode), ..} => {
                            match keycode {
                                Keycode::Escape | Keycode::Q => return UIAction::Quit,
                                Keycode::R => return UIAction::Reset,
                                Keycode::F => self.toggle_fullscreen(),
                                _ => {}
                            }
                        },
                        Event::Quit {..} => return UIAction::Quit,
                        _ => {},
                    }
                },
                None => return UIAction::Nothing,
            }
        }
    }
}
