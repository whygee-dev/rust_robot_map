pub mod robot {
    use std::collections::HashSet;

    use crate::{
        map::map::{DiscoveredCoord, Obstacle, Resource},
        shared::shared::Coord,
        station::station::Station,
    };

    pub struct Modules {
        miner: bool,
        energy_collector: bool,
        scientific_analyzer: bool,
    }

    impl Modules {
        pub fn new(miner: bool, energy_collector: bool, scientific_analyzer: bool) -> Modules {
            Modules {
                miner,
                energy_collector,
                scientific_analyzer,
            }
        }
    }

    pub struct RobotStorage {
        energy: u32,
        minerals: u32,
        pub discovered_coords: HashSet<DiscoveredCoord>,
    }

    pub struct Robot {
        id: u32,
        position: Coord,
        modules: Modules,
        storage: RobotStorage,
    }

    impl Robot {
        pub fn get_position(&self) -> &Coord {
            &self.position
        }

        pub fn get_storage(&self) -> &RobotStorage {
            &self.storage
        }

        pub fn new(id: u32, position: Coord, modules: Modules) -> Robot {
            Robot {
                id,
                position,
                modules,
                storage: RobotStorage {
                    energy: 0,
                    minerals: 0,
                    discovered_coords: HashSet::new(),
                },
            }
        }

        pub fn move_and_collect(
            &mut self,
            resources: &mut Vec<Resource>,
            obstacles: &Vec<Obstacle>,
            station: &Station,
            new_x: u32,
            new_y: u32,
        ) {
            resources
                .iter_mut()
                .filter(|r| *r.get_position() == self.position)
                .for_each(|collected_resource| {
                    if *collected_resource.get_energy() > 0 && self.modules.energy_collector {
                        self.storage.energy += *collected_resource.get_energy();
                        collected_resource.set_energy(0);
                    }
                    if *collected_resource.get_minerals() > 0 && self.modules.miner {
                        self.storage.minerals += *collected_resource.get_minerals();
                        collected_resource.set_minerals(0);
                    }
                    self.storage.discovered_coords.insert((
                        new_x,
                        new_y,
                        *collected_resource.get_scientific_interest()
                            && self.modules.scientific_analyzer,
                    ));
                });

            if new_x == station.get_position().0 && new_y == station.get_position().1 {
                station.store_energy(self.storage.energy);
                self.storage.energy = 0;

                station.store_minerals(self.storage.minerals);
                self.storage.minerals = 0;

                station.store_discovered_coords(&self.storage.discovered_coords);
                self.storage.discovered_coords.clear();
            }

            if !obstacles
                .iter()
                .find(|&&(obs_x, obs_y, _)| (obs_x, obs_y) == (new_x, new_y))
                .is_some()
            {
                self.position = (new_x, new_y);
            }
        }
    }
}
