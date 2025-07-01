extern crate sdl2; 

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Point, Rect};
use std::time::Duration;
 
pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
 
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.set_draw_color(Color::WHITE);
    // Roads
    let _= canvas.draw_line(Point::new(400, 0), Point::new(400,600));
    let _= canvas.draw_line(Point::new(350, 0), Point::new(350,600));
    let _= canvas.draw_line(Point::new(450, 0), Point::new(450,600));
    let _= canvas.draw_line(Point::new(0, 300), Point::new(800,300));
    let _= canvas.draw_line(Point::new(0, 250), Point::new(800,250));
    let _= canvas.draw_line(Point::new(0, 350), Point::new(800,350));
    // lights
    canvas.set_draw_color(Color::RED);
    let _left_top = canvas.draw_rect(Rect::new(318, 218, 30, 30));
    let _right_top = canvas.draw_rect(Rect::new(452, 218, 30, 30));
    let _left_bottom = canvas.draw_rect(Rect::new(318, 352, 30, 30));
    let _right_bottom = canvas.draw_rect(Rect::new(452, 352, 30, 30));
    
    // Cars
    canvas.set_draw_color(Color::BLUE);
    let _blue_car = canvas.fill_rect(Rect::new(360, 0, 30, 30));
    canvas.set_draw_color(Color::YELLOW);
    let _blue_car = canvas.fill_rect(Rect::new(410, 0, 30, 30));
    canvas.set_draw_color(Color::RGB(160, 32, 240));
    let _blue_car = canvas.fill_rect(Rect::new(360, 550, 30, 30));


    'running: loop {
        // canvas.clear();
        
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}