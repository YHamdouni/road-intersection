extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
// use sdl2::render::Canvas;
use sdl2::sys::{TopIf, Window, random};
use std::time::Duration;
use rand::Rng;

#[derive(Debug)]
enum Direction {
    Top,
    Down,
    Left,
    Right,
}
#[derive(Debug)]
enum Car_color {
    Blue,
    Yellow,
    Purple,
}
impl Car_color {
    pub fn random_c() -> Self {
        let mut rng = rand::rng();
        rng.random_range(0..2)
    }
}
#[derive(Debug)]
pub struct Car {
    pub x: i32,
    pub y: i32,
    pub dir: Direction,
    pub color: Car_color,
}
impl Car {
    pub fn new(x: i32, y: i32, dir: Direction, color: Car_color) -> Car {
        Car {
            x: x,
            y: y,
            dir: dir,
            color,
        }
    }
    pub fn move_car(&mut self) {
        match self.dir {
            Direction::Top => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x += 1,
            Direction::Right => self.x -= 1,
        }
    }
    pub fn redirect(&mut self) {}
}
pub fn main() {
    use Direction::*;
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

    // Cars
    // canvas.set_draw_color(Color::BLUE);
    // let _blue_car = canvas.fill_rect(Rect::new(360, 0, 30, 30));
    // canvas.set_draw_color(Color::YELLOW);
    // let _blue_car = canvas.fill_rect(Rect::new(410, 0, 30, 30));
    // canvas.set_draw_color(Color::RGB(160, 32, 240));
    // let _blue_car = canvas.fill_rect(Rect::new(360, 550, 30, 30));

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
            car.move_car();
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
                    canvas.set_draw_color(Color::RGB(160, 32, 240));
                    let car = Car::new(410, 560, Down, Car_color::random_c());
                    if cars_vec.len() > 0 {
                        let last_car = &cars_vec[cars_vec.len() - 1];
                        println!("last_car.y : {:?}", last_car.y);
                        if last_car.y < 510 {
                            if cars_vec.len() < 8 {
                                cars_vec.push(car);
                            }
                        }
                    } else {
                        if cars_vec.len() < 8 {
                            cars_vec.push(car);
                        }
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::DOWN),
                    ..
                } => {
                    println!("Keycode DOWN")
                }
                Event::KeyDown {
                    keycode: Some(Keycode::LEFT),
                    ..
                } => {
                    println!("Keycode LEFT")
                }
                Event::KeyDown {
                    keycode: Some(Keycode::RIGHT),
                    ..
                } => {
                    println!("Keycode RIGHT")
                }
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                    println!("Keycode R")
                }
                _ => {}
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
