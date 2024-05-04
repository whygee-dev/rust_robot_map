# Robot Simulation

## Description

This is a Rust project aimed at simulating the movement and behavior of robots in a virtual environment. It leverages Rust's safety, concurrency, and performance features, including the `rayon` and `parking_lot` crates, to deliver efficient and reliable functionality.

## Features

- **Movement Simulation:** Simulate the movement of robots in a 2D grid environment.
- **Robot Behavior:** Robots collect resources based on their modules, when they come back to the station they will deposit the resources.
- **Station:** Resources deposited by robots are in turn used to build new robots.
- **Concurrency:** Utilize parallel processing with `rayon`. Each robot is simulated in a separate thread for improved performance.
- **Thread Synchronization:** Ensure thread safety with `parking_lot` primitives.

## Installation

### Prerequisites

- Rust
- Cargo

### Instructions

1. Clone the repository
2. Run the project: `cargo run main`
3. (Optional) Run tests: `cargo test`

### Known Limitations

1. Robots only deposit resources at the station when they meet it by chance. There is no pathfinding logic implemented in regards to returning to the station, also robots have unlimited storage capacity.
2. The station does not have any functionality other than being a point of resource deposit and robot creation.
3. The grid environment is static and does not change over time.
