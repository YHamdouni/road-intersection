# 🚦 Traffic Intersection Simulation
A simulation project to solve a traffic congestion problem at a major city intersection. The goal is to create an intelligent traffic control strategy and visualize it using a graphical interface. This project is built with the SDL2 library in Rust.

## 🧠 Objectives
Simulate a two-road intersection with bi-directional traffic.

Control vehicle movement using traffic lights (only Red and Green).

Prevent vehicle collisions and manage traffic congestion.

Visualize vehicle movement, stopping, and routing behavior.

Allow user input to spawn new vehicles dynamically.

## 🏗️ Environment Description
1. Roads
Two roads crossing to form a 4-way intersection.

One lane in each direction.

Vehicles can:

Turn left

Go straight

Turn right

2. Traffic Lights
Positioned at each lane entrance.

Only two colors: Red and Green.

Custom traffic light control algorithm to:

Avoid vehicle collisions

Reduce congestion (no more than 8 cars waiting)

3. Vehicles
Each vehicle:

Has a fixed velocity

Maintains a safety distance from others

Obeys traffic light rules

Has a pre-determined route (left, right, straight)

Vehicles are color-coded based on their route (configurable).

No vehicle can change route once selected.

## 🎮 Controls
| Key     | Action                                   |
| ------- | ---------------------------------------- |
| ↑ Up    | Spawn vehicle moving from South to North |
| ↓ Down  | Spawn vehicle moving from North to South |
| → Right | Spawn vehicle moving from West to East   |
| ← Left  | Spawn vehicle moving from East to West   |
| `r`     | Spawn vehicle from a random direction    |
| `Esc`   | Exit the simulation                      |
## 🚫 Anti-spam: Vehicles can't be spawned if the safe distance constraint is violated.

## 🚧 Requirements
Rust installed (rustup)

SDL2 (bindings used: sdl2)

Dependencies installed via cargo
```
cargo build
cargo run
```
## 🎨 Bonus Features (Optional)
Vehicle animations and sprite rendering.

Animated traffic lights.

Visual assets from:

limezu

finalbossblue

The Spriters Resource

## 📁 Project Structure (Suggested)
```
css
traffic_sim/
├── src/
│   ├── main.rs
│   ├── vehicle.rs
│   ├── traffic_light.rs
│   ├── intersection.rs
│   └── input.rs
├── assets/
│   └── (sprites, images)
├── Cargo.toml
└── README.md
```
