extern crate sdl2;

use crate::gol::Grid;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::{FullscreenType, Window};
use sdl2::EventPump;

#[derive(PartialEq)]
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

    pub fn render_grid(&mut self, grid: &dyn Grid) {
        self.canvas.set_draw_color(Color::RGB(64, 0, 0));
        let (width, height) = self.canvas.output_size().unwrap();
        self.canvas
            .set_scale(
                width as f32 / grid.get_width() as f32,
                height as f32 / grid.get_height() as f32,
            )
            .unwrap();
        self.canvas.clear();
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        for x in 0..grid.get_width() {
            for y in 0..grid.get_height() {
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
                    let action = self.interpret_event(event);
                    if action != UIAction::Nothing {
                        return action;
                    }
                },
                None => return UIAction::Nothing,
            }
        }
    }

    #[inline(always)]
    fn interpret_event(&mut self, event: Event) -> UIAction {
        return match event {
            Event::KeyDown {keycode: Some(keycode), ..} => self.interpret_keycode(keycode),
            Event::Quit {..} => UIAction::Quit,
            _ => UIAction::Nothing
        }
    }

    #[inline(always)]
    fn interpret_keycode(&mut self, keycode: Keycode) -> UIAction {
        match keycode {
            Keycode::Escape | Keycode::Q => return UIAction::Quit,
            Keycode::R => return UIAction::Reset,
            Keycode::F => self.toggle_fullscreen(),
            _ => {}
        }

        return UIAction::Nothing;
    }
}
