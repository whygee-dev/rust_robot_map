pub mod robot {
    use std::collections::HashSet;

    use crate::{
        map::map::{Obstacle, Resource},
        shared::shared::Coord,
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
        scientific_interest: u32,
    }

    pub struct Robot {
        id: u32,
        pub position: Coord,
        modules: Modules,
        storage: RobotStorage,
        pub seen: HashSet<Coord>,
    }

    impl Robot {
        pub fn new(id: u32, position: Coord, modules: Modules) -> Robot {
            Robot {
                id,
                position,
                modules,
                storage: RobotStorage {
                    energy: 0,
                    minerals: 0,
                    scientific_interest: 0,
                },
                seen: HashSet::new(),
            }
        }

        pub fn move_and_collect(
            &mut self,
            resources: &mut Vec<Resource>,
            obstacles: &Vec<Obstacle>,
            new_x: u32,
            new_y: u32,
        ) {
            resources
                .iter_mut()
                .filter(|r| r.position == self.position)
                .for_each(|collected_resource| {
                    if collected_resource.energy > 0 && self.modules.energy_collector {
                        self.storage.energy += collected_resource.energy;
                        collected_resource.energy = 0
                    }
                    if collected_resource.minerals > 0 && self.modules.miner {
                        self.storage.minerals += collected_resource.minerals;
                        collected_resource.minerals = 0
                    }
                    if collected_resource.scientific_interest > 0
                        && self.modules.scientific_analyzer
                    {
                        self.storage.scientific_interest += collected_resource.scientific_interest;
                        collected_resource.scientific_interest = 0
                    }
                });

            if !obstacles
                .iter()
                .find(|&&(obs_x, obs_y, _)| (obs_x, obs_y) == (new_x, new_y))
                .is_some()
            {
                self.position = (new_x, new_y);
                self.seen.insert((new_x, new_y));
            }
        }
    }
}
