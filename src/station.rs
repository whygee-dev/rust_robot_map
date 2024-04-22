pub mod station {
    use std::collections::HashSet;

    use parking_lot::Mutex;
    use rand::{rngs::StdRng, Rng, SeedableRng as _};

    use crate::{
        map::map::DiscoveredCoord,
        robot::robot::{Modules, Robot},
        shared::shared::{generate_unique_id, Coord},
    };

    pub struct Station {
        position: Coord,
        stored_energy: Mutex<u32>,
        stored_minerals: Mutex<u32>,
        discovered_coords: Mutex<HashSet<DiscoveredCoord>>,
    }

    static ENERGY_PER_ROBOT: u32 = 1000;
    static MINERALS_PER_ROBOT: u32 = 1000;

    impl Station {
        pub fn new() -> Station {
            Station {
                position: (0, 0),
                stored_energy: Mutex::new(0),
                stored_minerals: Mutex::new(0),
                discovered_coords: Mutex::new(HashSet::new()),
            }
        }

        pub fn get_position(&self) -> &Coord {
            &self.position
        }

        pub fn store_energy(&self, energy: u32) {
            let mut stored_energy = self.stored_energy.lock();
            *stored_energy = stored_energy.checked_add(energy).unwrap();
        }

        pub fn store_minerals(&self, minerals: u32) {
            let mut stored_minerals = self.stored_minerals.lock();
            *stored_minerals = stored_minerals.checked_add(minerals).unwrap();
        }

        pub fn store_discovered_coords(&self, coords: &HashSet<DiscoveredCoord>) {
            let mut discovered_coords = self.discovered_coords.lock();

            for coord in coords {
                if !discovered_coords.contains(&coord) {
                    discovered_coords.insert(*coord);
                }
            }
        }

        pub fn create_robots_if_able(&self, _rng: &Mutex<StdRng>) -> Option<Vec<Robot>> {
            let mut stored_energy = self.stored_energy.lock();
            let mut stored_minerals = self.stored_minerals.lock();

            if stored_energy.lt(&ENERGY_PER_ROBOT) && stored_minerals.lt(&MINERALS_PER_ROBOT) {
                return None;
            }

            let mut robots = Vec::new();

            while stored_energy.ge(&ENERGY_PER_ROBOT) && stored_minerals.ge(&MINERALS_PER_ROBOT) {
                let drawed_nums;

                {
                    let mut rng = _rng.lock();

                    drawed_nums = rng.gen_range(0..=2);
                }

                robots.push(Robot::new(
                    generate_unique_id(_rng),
                    (0, 0),
                    Modules::new(drawed_nums == 0, drawed_nums == 1, drawed_nums == 2),
                ));

                *stored_energy = stored_energy.checked_sub(ENERGY_PER_ROBOT).unwrap();
                *stored_minerals = stored_minerals.checked_sub(MINERALS_PER_ROBOT).unwrap();
            }

            Some(robots)
        }
    }

    #[test]
    fn test_store_energy() {
        let station = Station {
            position: (0, 0),
            stored_energy: Mutex::new(100),
            stored_minerals: Mutex::new(0),
            discovered_coords: Mutex::new(HashSet::new()),
        };

        station.store_energy(50);

        let stored_energy = *station.stored_energy.lock();

        assert_eq!(stored_energy, 150);
    }

    #[test]
    fn test_store_minerals() {
        let station = Station {
            position: (0, 0),
            stored_energy: Mutex::new(0),
            stored_minerals: Mutex::new(100),
            discovered_coords: Mutex::new(HashSet::new()),
        };

        station.store_minerals(50);

        let stored_minerals = *station.stored_minerals.lock();

        assert_eq!(stored_minerals, 150);
    }

    #[test]
    fn test_store_discovered_coords() {
        let station = Station {
            position: (0, 0),
            stored_energy: Mutex::new(0),
            stored_minerals: Mutex::new(0),
            discovered_coords: Mutex::new(HashSet::new()),
        };

        let mut coords = HashSet::new();
        coords.insert((1, 2, false));
        coords.insert((3, 4, true));

        station.store_discovered_coords(&coords);

        let discovered_coords = station.discovered_coords.lock();

        assert_eq!(discovered_coords.len(), 2);
        assert!(discovered_coords.contains(&(1, 2, false)));
        assert!(discovered_coords.contains(&(3, 4, true)));
    }

    #[test]
    fn test_create_robots_if_able() {
        let station = Station {
            position: (0, 0),
            stored_energy: Mutex::new(2000),
            stored_minerals: Mutex::new(2000),
            discovered_coords: Mutex::new(HashSet::new()),
        };

        let rng = Mutex::new(StdRng::from_entropy());

        let robots = station.create_robots_if_able(&rng).unwrap();

        assert_eq!(robots.len(), 2);
        assert_eq!(robots[0].get_position(), &(0, 0));
        assert_eq!(robots[1].get_position(), &(0, 0));
    }
}
