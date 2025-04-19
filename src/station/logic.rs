use std::collections::HashMap;
use crate::map::Cell;
use crate::robot::RobotRole;
use crate::config::{EXPLORER_COST, COLLECTOR_COST, SCIENTIST_COST};

pub fn can_create_robot(resources: &mut HashMap<Cell, usize>) -> Option<RobotRole> {
    let energy = resources.get(&Cell::Energy).copied().unwrap_or(0);
    let mineral = resources.get(&Cell::Mineral).copied().unwrap_or(0);
    let science = resources.get(&Cell::Science).copied().unwrap_or(0);

    if energy >= EXPLORER_COST {
        *resources.entry(Cell::Energy).or_insert(0) -= EXPLORER_COST;
        Some(RobotRole::Explorer)
    } else if mineral >= COLLECTOR_COST {
        *resources.entry(Cell::Mineral).or_insert(0) -= COLLECTOR_COST;
        Some(RobotRole::Collector)
    } else if science >= SCIENTIST_COST {
        *resources.entry(Cell::Science).or_insert(0) -= SCIENTIST_COST;
        Some(RobotRole::Scientist)
    } else {
        None
    }
}
