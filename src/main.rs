mod map;
mod renderer;
mod robot;
mod shared;
mod simulation;
mod station;

use parking_lot::Mutex;
use rand::rngs::StdRng;
use rand::SeedableRng;

use simulation::simulation::start_simulation;

fn main() {
    let seed: u64 = 123;
    let rng = Mutex::new(StdRng::seed_from_u64(seed));

    start_simulation(rng, 20, 20, 20, 20);
}
