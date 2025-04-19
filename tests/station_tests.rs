use projet_essaim::station::Station;
use projet_essaim::map::Cell;
use projet_essaim::robot::{Robot, RobotRole, Direction};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_station_creation() {
        let station = Station::new(0, 0);
        assert_eq!(station.x, 0);
        assert_eq!(station.y, 0);
        assert!(station.discovered.is_empty());
        assert!(station.resources_collected.is_empty());
        assert_eq!(station.robots_created, 0);
        assert_eq!(station.scientific_discoveries, 0);
    }

    #[test]
    fn test_station_resource_collection() {
        let mut station = Station::new(0, 0);
        let resources = vec![Cell::Energy, Cell::Mineral, Cell::Science];
        station.receive_resources(resources);
        
        assert_eq!(station.resources_collected.get(&Cell::Energy), Some(&1));
        assert_eq!(station.resources_collected.get(&Cell::Mineral), Some(&1));
        assert_eq!(station.resources_collected.get(&Cell::Science), Some(&1));
    }

    #[test]
    fn test_station_explorer_tracking() {
        let mut station = Station::new(0, 0);
        let explorer = Robot::new(1, 1, Direction::North, RobotRole::Explorer);
        let explorers = vec![&explorer];
        
        station.update_explorer_positions(&explorers);
        assert_eq!(station.explorer_positions, vec![(1, 1)]);
    }
} 