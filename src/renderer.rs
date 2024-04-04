pub mod renderer {
    use std::process::Command;

    use crate::map::map::Map;

    pub fn render_map(map: &Map) {
        let _ = Command::new("clear").status();

        for y in 0..map.height {
            for x in 0..map.width {
                let is_station = x == map.station.position.0 && y == map.station.position.1;
                let is_robot = map
                    .robots
                    .lock()
                    .iter()
                    .any(|robot| robot.position == (x, y));

                let is_obstacle = map.obstacles.iter().any(|&(ox, oy, _)| ox == x && oy == y);

                if is_station {
                    print!("S ");
                } else if is_robot {
                    print!("R ");
                } else if is_obstacle {
                    print!("# ");
                } else {
                    print!(". ");
                }
            }

            println!();
        }
    }
}
