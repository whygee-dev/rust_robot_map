pub mod renderer {
    use colored::Colorize;
    use std::process::Command;

    use crate::map::map::Map;

    pub fn render_map(map: &Map) {
        let _ = Command::new("clear").status();

        for y in 0..*map.get_height() {
            for x in 0..*map.get_width() {
                let is_station = x == map.get_station().get_position().0
                    && y == map.get_station().get_position().1;
                let is_robot = map
                    .get_robots()
                    .lock()
                    .iter()
                    .any(|robot| *robot.get_position() == (x, y));

                let is_obstacle = map
                    .get_obstacles()
                    .iter()
                    .any(|&(ox, oy, _)| ox == x && oy == y);

                if is_station {
                    print!("{}", "S ".green());
                } else if is_robot {
                    print!("{}", "R ".blue());
                } else if is_obstacle {
                    print!("{}", "# ".red());
                } else {
                    print!("{}", ". ".white());
                }
            }

            println!();
        }
    }
}
