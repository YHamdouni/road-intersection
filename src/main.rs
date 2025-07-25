mod car;
mod lights;
pub use car::*;
pub use lights::*;
extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{ Point, Rect };

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Road Intersection", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut cars_vec: Vec<Car> = Vec::new();
    let mut lights = TrafficLights {
        lights_top: false,
        lights_down: false,
        lights_left: false,
        lights_right: false,
        time: Instant::now(),
        current_direction: Direction::Right,
        state: false,
    };
    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(Color::GREY);
        let _ = canvas.fill_rect(Rect::new(0, 250, 800, 100));
        let _ = canvas.fill_rect(Rect::new(350, 0, 100, 600));
        canvas.set_draw_color(Color::WHITE);
        // Roads
        let _ = canvas.draw_line(Point::new(400, 0), Point::new(400, 600));
        for i in (0..=600).step_by(15) {
            if i < 250 || i > 350 {
                canvas.set_draw_color(Color::GRAY);
                let _ = canvas.draw_line(Point::new(400, i), Point::new(400, i + 2));
            } else {
                canvas.set_draw_color(Color::GRAY);
                let _ = canvas.draw_line(Point::new(400, 250), Point::new(400, 350));
            }
        }
        canvas.set_draw_color(Color::WHITE);
        let _ = canvas.draw_line(Point::new(0, 300), Point::new(800, 300));
        for i in (0..=800).step_by(15) {
            if i < 350 || i > 450 {
                canvas.set_draw_color(Color::GRAY);
                let _ = canvas.draw_line(Point::new(i, 300), Point::new(i + 2, 300));
            } else {
                canvas.set_draw_color(Color::GRAY);
                let _ = canvas.draw_line(Point::new(350, 300), Point::new(450, 300));
            }
        }
        canvas.set_draw_color(Color::WHITE);
        let _ = canvas.draw_line(Point::new(350, 0), Point::new(350, 600));
        let _ = canvas.draw_line(Point::new(450, 0), Point::new(450, 600));
        let _ = canvas.draw_line(Point::new(0, 250), Point::new(800, 250));
        let _ = canvas.draw_line(Point::new(0, 350), Point::new(800, 350));

        traffic_lights_sys(&mut lights);
        // TOP light
        if lights.lights_top {
            canvas.set_draw_color(Color::GREEN);
        } else {
            canvas.set_draw_color(Color::RED);
        }
        let _ = canvas.fill_rect(Rect::new(318, 218, 30, 30));

        // RIGHT light
        if lights.lights_right {
            canvas.set_draw_color(Color::GREEN);
        } else {
            canvas.set_draw_color(Color::RED);
        }
        let _ = canvas.fill_rect(Rect::new(318, 352, 30, 30));

        // DOWN light
        if lights.lights_down {
            canvas.set_draw_color(Color::GREEN);
        } else {
            canvas.set_draw_color(Color::RED);
        }
        let _ = canvas.fill_rect(Rect::new(452, 352, 30, 30));

        // LEFT light
        if lights.lights_left {
            canvas.set_draw_color(Color::GREEN);
        } else {
            canvas.set_draw_color(Color::RED);
        }
        let _ = canvas.fill_rect(Rect::new(452, 218, 30, 30));
        let copy_cars = cars_vec.clone();
        for car in &mut cars_vec {
            canvas.set_draw_color(car.color);
            traffic_lights(car, &mut lights);
            if car.moving {
                if !car.next_car(&copy_cars) {
                    car.move_car();
                }
            }
            if !car.turned {
                car.redirect();
            }
            let _ = canvas.fill_rect(Rect::new(car.x, car.y, 30, 30));
        }
        cars_vec.retain(|car| car.y <= 630 && car.y >= -30 && car.x <= 830 && car.x >= -30);
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                Event::KeyDown { keycode: Some(Keycode::UP), .. } => {
                    key_up(&mut cars_vec);
                }
                Event::KeyDown { keycode: Some(Keycode::DOWN), .. } => {
                    key_down(&mut cars_vec);
                }
                Event::KeyDown { keycode: Some(Keycode::LEFT), .. } => {
                    key_left(&mut cars_vec);
                }
                Event::KeyDown { keycode: Some(Keycode::RIGHT), .. } => {
                    key_right(&mut cars_vec);
                }
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    key_r(&mut cars_vec);
                }
                _ => {}
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
