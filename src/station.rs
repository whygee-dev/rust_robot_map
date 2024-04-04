pub mod station {
    use std::collections::HashSet;

    use parking_lot::Mutex;
    use rand::{rngs::StdRng, Rng};

    use crate::{
        map::map::DiscoveredCoord,
        robot::robot::{Modules, Robot},
        shared::shared::{generate_unique_id, Coord},
    };

    pub struct Station {
        pub position: Coord,
        pub stored_energy: Mutex<u32>,
        pub stored_minerals: Mutex<u32>,
        pub discovered_coords: Mutex<HashSet<DiscoveredCoord>>,
    }

    static ENERGY_PER_ROBOT: u32 = 1000;
    static MINERALS_PER_ROBOT: u32 = 1000;

    impl Station {
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
}
