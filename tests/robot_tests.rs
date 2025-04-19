use projet_essaim::robot::{Robot, Direction, RobotRole};
use projet_essaim::map::Map;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_robot_creation() {
        let robot = Robot::new(0, 0, Direction::North, RobotRole::Explorer);
        assert_eq!(robot.x, 0);
        assert_eq!(robot.y, 0);
        assert_eq!(robot.direction, Direction::North);
        assert_eq!(robot.role, RobotRole::Explorer);
    }

    #[test]
    fn test_robot_turn() {
        let mut robot = Robot::new(0, 0, Direction::North, RobotRole::Explorer);
        robot.turn_left();
        assert_eq!(robot.direction, Direction::West);
        robot.turn_right();
        assert_eq!(robot.direction, Direction::North);
    }

    #[test]
    fn test_robot_movement() {
        let mut robot = Robot::new(0, 0, Direction::East, RobotRole::Explorer);
        let map = Map::new(10, 10, 42);
        robot.move_forward(&map);
        assert_eq!(robot.x, 1);
        assert_eq!(robot.y, 0);
    }
} 