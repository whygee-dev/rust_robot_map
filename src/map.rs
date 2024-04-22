pub mod map {
    use std::collections::HashSet;

    use parking_lot::Mutex;
    use rand::{rngs::StdRng, Rng, RngCore};

    use crate::{
        robot::robot::{Modules, Robot},
        shared::shared::{add, generate_unique_ids, Coord},
        station::station::Station,
    };
    use noise::{NoiseFn, Perlin};

    pub enum ObstacleType {
        Rock,
        Pit,
    }

    const INITIAL_ROBOTS_COUNT: usize = 3;

    #[derive(Debug)]
    pub struct Resource {
        pub position: Coord,
        pub energy: u32,
        pub minerals: u32,
        pub scientific_interest: bool,
    }

    pub type Obstacle = (u32, u32, ObstacleType);
    pub type DiscoveredCoord = (u32, u32, bool); // (x, y is_scientific_interest)

    pub struct Map {
        pub width: u32,
        pub height: u32,
        pub robots: Mutex<Vec<Robot>>,
        pub resources: Mutex<Vec<Resource>>,
        pub obstacles: Vec<Obstacle>,
        pub station: Station,
    }

    impl Map {
        pub fn get_next_coords(
            &self,
            current_x: u32,
            current_y: u32,
            discovered_coords: &HashSet<DiscoveredCoord>,
            obstacles: &Vec<Obstacle>,
            rng: &Mutex<StdRng>,
        ) -> Coord {
            let directions = Vec::from([(0, 1), (0, -1), (1, 0), (-1, 0)]);

            let mut valid_directions = Vec::new();

            for &(dx, dy) in &directions {
                let next_x = add(current_x, dx, self.width);
                let next_y = add(current_y, dy, self.height);

                if !discovered_coords
                    .iter()
                    .any(|(x, y, _)| *x == next_x && *y == next_y)
                    && !obstacles
                        .iter()
                        .any(|&(ox, oy, _)| ox == next_x && oy == next_y)
                {
                    valid_directions.push((dx, dy));
                }
            }

            let directions_to_choose_from = if valid_directions.is_empty() {
                &directions
            } else {
                &valid_directions
            };

            let index = rng.lock().gen_range(0..directions_to_choose_from.len());
            let (dx, dy) = directions_to_choose_from[index];
            let next_x = add(current_x, dx, self.width);
            let next_y = add(current_y, dy, self.height);

            (next_x, next_y)
        }
    }

    pub fn generate_map(width: u32, height: u32, _rng: &Mutex<StdRng>) -> Map {
        let mut resources = Vec::new();
        let mut obstacles = Vec::new();
        let mut robots = Vec::new();

        {
            let mut rng = _rng.lock();
            let perlin = Perlin::new(rng.next_u32());

            for x in 0..width {
                for y in 0..height {
                    let energy = rng.gen_range(0..=100);
                    let minerals = rng.gen_range(0..=100);
                    let scientific_interest = rng.gen_bool(0.1);

                    resources.push(Resource {
                        position: (x, y),
                        energy,
                        minerals,
                        scientific_interest,
                    });

                    let noise_value = perlin.get([x as f64 / 5.0, y as f64 / 5.0]);

                    if noise_value > 0.3 && x != 0 && y != 0 {
                        let obstacle_type = if noise_value > 0.6 {
                            ObstacleType::Rock
                        } else {
                            ObstacleType::Pit
                        };

                        obstacles.push((x, y, obstacle_type));
                    }
                }
            }
        }

        let unique_ids = generate_unique_ids(&_rng, INITIAL_ROBOTS_COUNT);
        let mut unique_ids_iter = unique_ids.iter();

        for i in 0..INITIAL_ROBOTS_COUNT {
            robots.push(Robot::new(
                *unique_ids_iter.next().unwrap(),
                (0, 0),
                Modules::new(
                    i == 0 || i % 3 == 0,
                    i == 1 || i % 3 == 1,
                    i == 2 || i % 3 == 2,
                ),
            ));
        }

        Map {
            width,
            height,
            robots: Mutex::new(robots),
            resources: Mutex::new(resources),
            obstacles,
            station: Station {
                position: (0, 0),
                stored_energy: Mutex::new(0),
                stored_minerals: Mutex::new(0),
                discovered_coords: Mutex::new(HashSet::new()),
            },
        }
    }
}
