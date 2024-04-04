pub mod simulation {
    use std::{thread, time::Duration};

    use crate::map::map::generate_map;
    use crate::renderer::renderer::render_map;
    use parking_lot::Mutex;
    use rand::{rngs::StdRng, Rng};
    use rayon::prelude::*;

    pub fn start_simulation(
        _rng: Mutex<StdRng>,
        min_width: u32,
        max_width: u32,
        min_height: u32,
        max_height: u32,
        steps: u32,
    ) {
        let map_width: u32;
        let map_height: u32;

        {
            let mut rng = _rng.lock();
            map_width = rng.gen_range(min_width..=max_width);
            map_height = rng.gen_range(min_height..=max_height);
        }

        let map = generate_map(map_width, map_height, &_rng);

        println!("{} {}", map_width, map_height);

        for step in 0..steps {
            println!("Step {step}");

            {
                render_map(&map);
            }

            {
                map.robots.lock().par_iter_mut().for_each(|robot| {
                    let (current_x, current_y) = robot.position;
                    let (new_x, new_y) = map.get_next_coords(
                        current_x,
                        current_y,
                        &robot.seen,
                        &map.obstacles,
                        &_rng,
                    );

                    robot.move_and_collect(&mut map.resources.lock(), &map.obstacles, new_x, new_y);
                });
            }

            thread::sleep(Duration::from_millis(100));
        }
    }
}
