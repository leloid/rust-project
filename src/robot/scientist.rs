use crate::map::{Map, Cell};
use crate::station::Station;
use crate::robot::Robot;

impl Robot {
    pub fn act_as_scientist(&mut self, map: &mut Map, station_x: usize, station_y: usize, station: &mut Station) {
        self.vision(map, 2, station);

        let current_cell = map.grid[self.y][self.x];
        if current_cell == Cell::Science && self.collected.len() < 1 {
            self.collected.push(current_cell);
            map.grid[self.y][self.x] = Cell::Empty;
            println!("Scientist collected a science resource! Total collected: {}", self.collected.len());
        }

        if self.collected.len() >= 1 {
            if self.x == station_x && self.y == station_y {
                println!("Scientist depositing {} science resources at station", self.collected.len());
                station.receive_resources(self.collected.drain(..).collect());
            } else {
                self.move_dijkstra_to(map, station_x, station_y);
            }
        } else {
            if let Some((target_x, target_y)) = self.find_nearest_scientist_position(map) {
                self.move_dijkstra_to(map, target_x, target_y);
            } else {
                self.move_random(map);
            }
        }
    }
}
