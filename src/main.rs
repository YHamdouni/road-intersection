mod create;
pub use create::*;
extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use std::time::Duration;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut cars_vec: Vec<Car> = Vec::new();
    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(Color::WHITE);
        // Roads
        let _ = canvas.draw_line(Point::new(400, 0), Point::new(400, 600));
        let _ = canvas.draw_line(Point::new(350, 0), Point::new(350, 600));
        let _ = canvas.draw_line(Point::new(450, 0), Point::new(450, 600));
        let _ = canvas.draw_line(Point::new(0, 300), Point::new(800, 300));
        let _ = canvas.draw_line(Point::new(0, 250), Point::new(800, 250));
        let _ = canvas.draw_line(Point::new(0, 350), Point::new(800, 350));
        // lights
        canvas.set_draw_color(Color::RED);
        let _ = canvas.draw_rect(Rect::new(318, 218, 30, 30));
        let _ = canvas.draw_rect(Rect::new(452, 218, 30, 30));
        let _ = canvas.draw_rect(Rect::new(318, 352, 30, 30));
        let _ = canvas.draw_rect(Rect::new(452, 352, 30, 30));

        canvas.set_draw_color(Color::BLUE);
        for car in &mut cars_vec {
            canvas.set_draw_color(car.color);
            car.move_car();
            car.redirect();
            let _ = canvas.fill_rect(Rect::new(car.x, car.y, 30, 30));
        }
        cars_vec.retain(|car| car.y <= 630 && car.y >= -30 && car.x <= 830 && car.x >= -30);
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::UP),
                    ..
                } => {
                    key_up(&mut cars_vec);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::DOWN),
                    ..
                } => {
                    key_down(&mut cars_vec);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::LEFT),
                    ..
                } => {
                    key_left(&mut cars_vec);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::RIGHT),
                    ..
                } => {
                    key_right(&mut cars_vec);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                    key_r(&mut cars_vec);
                }
                _ => {}
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
