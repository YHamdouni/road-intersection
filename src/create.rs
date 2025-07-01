use std::f32::DIGITS;

use rand::Rng;
use sdl2::pixels::Color;

#[derive(Debug, PartialEq)]
pub enum Direction {
    Top,
    Down,
    Left,
    Right,
}
#[derive(Debug, PartialEq)]
pub struct Car {
    pub x: i32,
    pub y: i32,
    pub dir: Direction,
    pub color: Color,
    pub turned: bool,
}
impl Car {
    pub fn new(x: i32, y: i32, dir: Direction, color: Color) -> Car {
        Car {
            x: x,
            y: y,
            dir: dir,
            color,
            turned: false,
        }
    }
    pub fn move_car(&mut self) {
        match self.dir {
            Direction::Top => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }
    pub fn redirect(&mut self) {
        //from direction Top:
        if self.dir == Direction::Top{
                    if self.x == 360 && self.y == 260 && self.color == Color::RGB(160, 32, 240){
            self.dir = Direction::Left;
            self.turned = true;
        }else if self.x == 360 && self.y == 310 && self.color == Color::YELLOW{
            self.dir = Direction::Right;
            self.turned = true;
        }
        }

        //from direction Down :
        else if self.dir == Direction::Down{
            if self.y == 260 && self.x == 410 && self.color == Color::YELLOW{
            self.dir = Direction::Left;
            self.turned = true;
        }else if self.y == 310 && self.x == 410 && self.color == Color::RGB(160, 32, 240){
            self.dir = Direction::Right;
            self.turned = true;
        }
        }
        
        // from direction right:
        else if self.dir == Direction::Right{
            if self.x == 360 && self.y == 310 && self.color == Color::RGB(160, 32, 240) && self.dir == Direction::Right{
            self.dir = Direction::Top;
            self.turned = true;
        } else if self.x == 410 && self.y == 310 && self.color == Color::YELLOW && self.dir == Direction::Right{
            self.dir = Direction::Down;
            self.turned = true;
        }
        }
        
        // from direction left:
        else if self.dir == Direction::Left{
        if self.x == 360 && self.y == 260 && self.color == Color::YELLOW {
            self.dir = Direction::Top;
            self.turned = true;
        } else if self.x == 410 && self.y == 260 && self.color == Color::RGB(160, 32, 240) {
            self.dir = Direction::Down;
            self.turned = true;
        }
        }
        

    }

    pub fn random_c() -> Color {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..=2) {
            0 => Color::BLUE,
            1 => Color::YELLOW,
            _ => Color::RGB(160, 32, 240),
        }
    }
}

pub fn key_up(cars_vec: &mut Vec<Car>) {
    let car = Car::new(410, 560, Direction::Down, Car::random_c());
    if cars_vec.len() > 0 {
        let last_car = &cars_vec[cars_vec.len() - 1];
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
pub fn key_down(cars_vec: &mut Vec<Car>) {
    let car = Car::new(360, 0, Direction::Top, Car::random_c());
    if cars_vec.len() > 0 {
        let last_car = &cars_vec[cars_vec.len() - 1];
        if last_car.y > 60 {
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
pub fn key_left(cars_vec: &mut Vec<Car>) {
    //0 310
    let car = Car::new(0, 310, Direction::Right, Car::random_c());
    if cars_vec.len() > 0 {
        let last_car = &cars_vec[cars_vec.len() - 1];
        if last_car.x > 60 {
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
pub fn key_right(cars_vec: &mut Vec<Car>) {
    let car = Car::new(800, 260, Direction::Left, Car::random_c());
    if cars_vec.len() > 0 {
        let last_car = &cars_vec[cars_vec.len() - 1];
        if last_car.x < 740 {
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
pub fn key_r(cars_vec: &mut Vec<Car>) {
    let vec: Vec<fn(&mut Vec<Car>)> = vec![key_up, key_down, key_left, key_right];
    let mut rng = rand::thread_rng();
    let choice = rng.gen_range(0..=3);
    vec[choice](cars_vec); // Call only the randomly chosen function
}
