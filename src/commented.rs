// use rand::Rng;
// use sdl2::pixels::Color;

// #[derive(Debug, PartialEq)]
// pub enum Direction {
//     Top,
//     Down,
//     Left,
//     Right,
// }
// #[derive(Debug, PartialEq)]
// pub struct Car {
//     pub x: i32,
//     pub y: i32,
//     pub dir: Direction,
//     pub color: Color,
// }
// impl Car {
//     pub fn new(x: i32, y: i32, dir: Direction, color: Color) -> Car {
//         Car {
//             x: x,
//             y: y,
//             dir: dir,
//             color,
//         }
//     }
//     pub fn move_car(&mut self) {
//         match self.dir {
//             Direction::Top => self.y += 1,
//             Direction::Down => self.y -= 1,
//             Direction::Left => self.x -= 1,
//             Direction::Right => self.x += 1,
//         }
//     }
//     pub fn redirect(&mut self) {
//         if (self.color == Color::RGB(160, 32, 240) && self.y == 310 && self.x == 410)
//             || (self.x == 360 && self.y == 310 && self.color == Color::YELLOW)
//         {
//             self.dir = Direction::Right;
//         } else if (self.y == 260 && self.x == 410 && self.color == Color::YELLOW)
//             || (self.x == 360 && self.y == 260 && self.color == Color::RGB(160, 32, 240))
//         {
//             self.dir = Direction::Left;
//         }
//     }

//     pub fn random_c() -> Color {
//         let mut rng = rand::thread_rng();
//         match rng.gen_range(0..=2) {
//             0 => Color::BLUE,
//             1 => Color::YELLOW,
//             _ => Color::RGB(160, 32, 240),
//         }
//     }
// }

// pub fn add_car(cars_vec: &mut Vec<Car>, x: i32, y: i32, dir: Direction, safedis: i32) {
//     let car = Car::new(x, y, dir, Car::random_c());
//     if cars_vec.len() > 0 {
//         let last_car = &cars_vec[cars_vec.len() - 1];
//         if last_car.y > safedis {
//             if cars_vec.len() < 8 {
//                 cars_vec.push(car);
//             }
//         }
//     } else {
//         if cars_vec.len() < 8 {
//             cars_vec.push(car);
//         }
//     }
// }

// pub fn key_up(cars_vec: &mut Vec<Car>) {
//     // add_car(cars_vec, 410, 560, Direction::Down, 510);
//     let car = Car::new(410, 560, Direction::Down, Car::random_c());
//     if cars_vec.len() > 0 {
//         let last_car = getcar(cars_vec, Direction::Down).unwrap();
//         if last_car.y < 510 {
//             if cars_vec.len() < 8 {
//                 cars_vec.push(car);
//             }
//         } else {
//             if cars_vec.len() < 8 {
//                 cars_vec.push(car);
//             }
//         }
//     } else {
//         if cars_vec.len() < 8 {
//             cars_vec.push(car);
//         }
//     }
// }
// pub fn key_down(cars_vec: &mut Vec<Car>) {
//     // add_car(cars_vec, 360, 0, Direction::Top, 50);
//     let car = Car::new(360, 0, Direction::Top, Car::random_c());
//     if cars_vec.len() > 0 {
//         let last_car = getcar(cars_vec, Direction::Top).unwrap();
//         if last_car.y > 50 {
//             if cars_vec.len() < 8 {
//                 cars_vec.push(car);
//             }
//         } else {
//             if cars_vec.len() < 8 {
//                 cars_vec.push(car);
//             }
//         }
//     } else {
//         if cars_vec.len() < 8 {
//             cars_vec.push(car);
//         }
//     }
//     // println!("Keycode DOWN");
// }
// pub fn key_left(cars_vec: &mut Vec<Car>) {
//     // add_car(cars_vec, 0, 310, Direction::Right, 60);
//     let car = Car::new(0, 310, Direction::Right, Car::random_c());
//     if cars_vec.len() > 0 {
//         let last_car = getcar(cars_vec, Direction::Right).unwrap();
//         if last_car.x > 60 {
//             if cars_vec.len() < 8 {
//                 cars_vec.push(car);
//             }
//         }else{
//             if cars_vec.len() < 8 {
//                 cars_vec.push(car);
//             }
//         }
//     } else {
//         if cars_vec.len() < 8 {
//             cars_vec.push(car);
//         }
//     }
//     // println!("Keycode LEFT");
// }
// pub fn key_right(cars_vec: &mut Vec<Car>) {
//     let car = Car::new(600, 260, Direction::Left, Car::random_c());
//     if cars_vec.len() > 0 {
//         let last_car = getcar(cars_vec, Direction::Left).unwrap();
//         if last_car.x < 60 {
//             if cars_vec.len() < 8 {
//                 cars_vec.push(car);
//             }
//         }else{
//             if cars_vec.len() < 8 {
//                 cars_vec.push(car);
//             }
//         }
//     } else {
//         if cars_vec.len() < 8 {
//             cars_vec.push(car);
//         }
//     }

// }
// pub fn key_r(cars_vec: &mut Vec<Car>) {
//     let vec: Vec<fn(&mut Vec<Car>)> = vec![key_up, key_down, key_left, key_right];
//     let mut rng = rand::thread_rng();
//     let choice = rng.gen_range(0..=3);
//     vec[choice](cars_vec); // Call only the randomly chosen function
// }

// pub fn getcar(cars_vec: &mut Vec<Car>, dir: Direction) -> Option<&mut Car> {
//     for car in cars_vec {
//         if car.dir == dir {
//             return Some(car);
//         }
//     }
//     None
// }
