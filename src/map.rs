pub mod map {
    use std::collections::HashSet;

    use parking_lot::Mutex;
    use rand::{rngs::StdRng, Rng, RngCore};

    use crate::robot::robot::{Modules, Robot};
    use crate::shared::shared::Coord;
    use noise::{NoiseFn, Perlin};

    pub enum ObstacleType {
        Rock,
        Pit,
    }

    pub struct Resource {
        pub position: Coord,
        pub energy: u32,
        pub minerals: u32,
        pub scientific_interest: u32,
    }

    fn add(u: u32, i: i32, size: u32) -> u32 {
        if u == 0 && i.is_negative() {
            size - i.wrapping_abs() as u32 % size
        } else {
            let result = if i.is_negative() {
                u.wrapping_sub(i.wrapping_abs() as u32)
            } else {
                u.wrapping_add(i as u32)
            };

            if result >= size {
                result % size
            } else {
                result
            }
        }
    }

    pub type Obstacle = (u32, u32, ObstacleType);

    pub struct Map {
        pub width: u32,
        pub height: u32,
        pub robots: Mutex<Vec<Robot>>,
        pub resources: Mutex<Vec<Resource>>,
        pub obstacles: Vec<Obstacle>,
    }

    impl Map {
        pub fn get_next_coords(
            &self,
            current_x: u32,
            current_y: u32,
            seen: &HashSet<Coord>,
            obstacles: &Vec<Obstacle>,
            rng: &Mutex<StdRng>,
        ) -> Coord {
            let directions = Vec::from([(0, 1), (0, -1), (1, 0), (-1, 0)]);

            let mut valid_directions = Vec::new();

            for &(dx, dy) in &directions {
                let next_x = add(current_x, dx, self.width);
                let next_y = add(current_y, dy, self.height);

                if !seen.contains(&(next_x, next_y))
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

    fn generate_unique_ids(_rng: &Mutex<StdRng>, count: usize) -> HashSet<u32> {
        let mut unique_ids = HashSet::new();
        let mut rng = _rng.lock();

        while unique_ids.len() < count {
            let id = rng.next_u32();
            unique_ids.insert(id);
        }

        unique_ids
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
                    let scientific_interest = rng.gen_range(0..=100);

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

        let unique_ids = generate_unique_ids(&_rng, 6);

        let mut iter = unique_ids.iter();

        for _ in 0..2 {
            robots.push(Robot::new(
                *iter.next().unwrap(),
                (0, 0),
                Modules::new(true, false, false),
            ));
        }

        for _ in 0..2 {
            robots.push(Robot::new(
                *iter.next().unwrap(),
                (0, 0),
                Modules::new(false, true, false),
            ));
        }

        for _ in 0..2 {
            robots.push(Robot::new(
                *iter.next().unwrap(),
                (0, 0),
                Modules::new(false, false, true),
            ));
        }

        Map {
            width,
            height,
            robots: Mutex::new(robots),
            resources: Mutex::new(resources),
            obstacles,
        }
    }
}
