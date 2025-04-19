use crate::map::Map;
use crate::station::Station;
use crate::robot::Robot;

impl Robot {
    pub fn act_as_explorer(&mut self, map: &mut Map, station_x: usize, station_y: usize, station: &mut Station) {
        self.vision(map, 2, station);
        if self.x == station_x && self.y == station_y {
            self.move_smart_towards_unknown_with_others(map, station.get_explorer_positions());
        } else {
            self.move_smart_towards_unknown_with_others(map, station.get_explorer_positions());
        }
    }
}
