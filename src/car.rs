pub use std::time::{ Duration, Instant };
use rand::Rng;
use sdl2::{ pixels::Color };

#[derive(Debug, PartialEq, Clone)]
pub enum Direction {
    Top,
    Down,
    Left,
    Right,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Car {
    pub x: i32,
    pub y: i32,
    pub dir: Direction,
    pub color: Color,
    pub turned: bool,
    pub moving: bool,
}
impl Car {
    pub fn new(x: i32, y: i32, dir: Direction, color: Color) -> Car {
        Car {
            x: x,
            y: y,
            dir: dir,
            color,
            turned: false,
            moving: false,
        }
    }
    pub fn move_car(&mut self) {
        match self.dir {
            Direction::Top => {
                self.y += 1;
            }
            Direction::Down => {
                self.y -= 1;
            }
            Direction::Left => {
                self.x -= 1;
            }
            Direction::Right => {
                self.x += 1;
            }
        }
    }
    pub fn redirect(&mut self) {
        //from direction Top:
        if self.dir == Direction::Top {
            if self.x == 360 && self.y == 260 && self.color == Color::RGB(160, 32, 240) {
                self.dir = Direction::Left;
                self.turned = true;
            } else if self.x == 360 && self.y == 310 && self.color == Color::YELLOW {
                self.dir = Direction::Right;
                self.turned = true;
            }
        } else if
            //from direction Down :
            self.dir == Direction::Down
        {
            if self.y == 260 && self.x == 410 && self.color == Color::YELLOW {
                self.dir = Direction::Left;
                self.turned = true;
            } else if self.y == 310 && self.x == 410 && self.color == Color::RGB(160, 32, 240) {
                self.dir = Direction::Right;
                self.turned = true;
            }
        } else if
            // from direction right:
            self.dir == Direction::Right
        {
            if
                self.x == 360 &&
                self.y == 310 &&
                self.color == Color::RGB(160, 32, 240) &&
                self.dir == Direction::Right
            {
                self.dir = Direction::Top;
                self.turned = true;
            } else if
                self.x == 410 &&
                self.y == 310 &&
                self.color == Color::YELLOW &&
                self.dir == Direction::Right
            {
                self.dir = Direction::Down;
                self.turned = true;
            }
        } else if
            // from direction left:
            self.dir == Direction::Left
        {
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

    pub fn next_car(&self, cars_iter: &Vec<Car>) -> bool {
        const SAFE_DISTANCE: i32 = 65;
        for other_car in cars_iter {
            if other_car.dir == self.dir {
                match self.dir {
                    Direction::Top => {
                        if other_car.y > self.y && other_car.y - self.y <= SAFE_DISTANCE {
                            return true;
                        }
                    }
                    Direction::Down => {
                        if other_car.y < self.y && self.y - other_car.y <= SAFE_DISTANCE {
                            return true;
                        }
                    }
                    Direction::Left => {
                        if other_car.x < self.x && self.x - other_car.x <= SAFE_DISTANCE {
                            return true;
                        }
                    }
                    Direction::Right => {
                        if other_car.x > self.x && other_car.x - self.x <= SAFE_DISTANCE {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
}
pub fn get_last_car_dir(cars_vec: &mut Vec<Car>, dir: Direction) -> Option<Car> {
    cars_vec
        .iter()
        .rev()
        .find(|car| car.dir == dir)
        .cloned()
}

pub fn push_car(
    cars_vec: &mut Vec<Car>,
    car: Car,
    check_y: Option<i32>,
    check_x: Option<i32>,
    dir: Direction
) {
    const MAX_CARS: usize = 200;

    let can_push = if cars_vec.is_empty() {
        true
    } else {
        let last_down_car = get_last_car_dir(cars_vec, dir);
        match last_down_car {
            Some(last_car) => {
                match (check_y, check_x) {
                    (Some(y_limit), None) => {
                        if car.dir == Direction::Down {
                            last_car.y < y_limit
                        } else {
                            last_car.y > y_limit
                        }
                    }
                    (None, Some(x_limit)) => {
                        if car.dir == Direction::Right {
                            last_car.x > x_limit
                        } else {
                            last_car.x < x_limit
                        }
                    }
                    _ => false,
                }
            }
            None => true,
        }
    };

    if can_push && cars_vec.len() < MAX_CARS {
        cars_vec.push(car);
    }
}

pub fn key_up(cars_vec: &mut Vec<Car>) {
    let car = Car::new(410, 560, Direction::Down, Car::random_c());
    push_car(cars_vec, car, Some(510), None, Direction::Down);
}

pub fn key_down(cars_vec: &mut Vec<Car>) {
    let car = Car::new(360, 0, Direction::Top, Car::random_c());
    push_car(cars_vec, car, Some(60), None, Direction::Top);
}

pub fn key_left(cars_vec: &mut Vec<Car>) {
    let car = Car::new(0, 310, Direction::Right, Car::random_c());
    push_car(cars_vec, car, None, Some(60), Direction::Right);
}

pub fn key_right(cars_vec: &mut Vec<Car>) {
    let car = Car::new(800, 260, Direction::Left, Car::random_c());
    push_car(cars_vec, car, None, Some(740), Direction::Left);
}

pub fn key_r(cars_vec: &mut Vec<Car>) {
    let vec: Vec<fn(&mut Vec<Car>)> = vec![key_up, key_down, key_left, key_right];
    let mut rng = rand::thread_rng();
    let choice = rng.gen_range(0..=3);
    (vec[choice])(cars_vec);
}

pub fn can_move(cars_vec: &mut Vec<Car>, new_car: Car) -> bool {
    match new_car.dir {
        Direction::Top => {
            let _is = cars_vec
                .iter()
                .rev()
                .find(|car| car == car)
                .cloned();
        }
        Direction::Down => {}
        Direction::Left => {}
        Direction::Right => {}
    }
    false
}

