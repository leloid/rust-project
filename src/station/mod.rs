pub mod logic;

use crate::map::Cell;
use crate::robot::{Robot, RobotRole, Direction};
use crate::config::{EXPLORER_COST, COLLECTOR_COST, SCIENTIST_COST};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Station {
    pub x: usize,
    pub y: usize,
    pub discovered: HashMap<(usize, usize), Cell>,
    pub resources_collected: HashMap<Cell, usize>,
    pub robots_created: usize,
    pub scientific_discoveries: usize,
    pub explorer_positions: Vec<(usize, usize)>,
}

impl Station {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            discovered: HashMap::new(),
            resources_collected: HashMap::new(),
            robots_created: 0,
            scientific_discoveries: 0,
            explorer_positions: Vec::new(),
        }
    }

    pub fn update_explorer_positions(&mut self, explorers: &[&Robot]) {
        self.explorer_positions.clear();
        for explorer in explorers {
            if explorer.role == RobotRole::Explorer {
                self.explorer_positions.push((explorer.x, explorer.y));
            }
        }
    }

    pub fn get_explorer_positions(&self) -> &[(usize, usize)] {
        &self.explorer_positions
    }

    pub fn receive_resources(&mut self, collected_cells: Vec<Cell>) {
        for cell in collected_cells {
            *self.resources_collected.entry(cell).or_insert(0) += 1;
        }
    }

    pub fn maybe_create_robot(&mut self) -> Option<Robot> {
        use logic::can_create_robot;

        if let Some(role) = can_create_robot(&mut self.resources_collected) {
            self.robots_created += 1;
            println!("Station created a new {:?} robot!", role);
            return Some(Robot::new(self.x, self.y, Direction::North, role));
        }

        None
    }
}
