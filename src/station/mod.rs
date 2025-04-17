use crate::map::Cell;
use crate::robot::{Robot, RobotRole, Direction};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Station {
    pub x: usize,
    pub y: usize,
    pub discovered: HashMap<(usize, usize), Cell>, // fusion des cartes
    pub resources_collected: HashMap<Cell, usize>,   // resources collected by collectors
    pub robots_created: usize,
    pub scientific_discoveries: usize,
    pub explorer_positions: Vec<(usize, usize)>,   // Keep track of explorer positions
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

    // Update explorer positions
    pub fn update_explorer_positions(&mut self, explorers: &[&Robot]) {
        self.explorer_positions.clear();
        for explorer in explorers {
            if explorer.role == RobotRole::Explorer {
                self.explorer_positions.push((explorer.x, explorer.y));
            }
        }
    }

    // Get explorer positions
    pub fn get_explorer_positions(&self) -> &[(usize, usize)] {
        &self.explorer_positions
    }

    pub fn receive_resources(&mut self, collected_cells: Vec<Cell>) {
        for cell in collected_cells {
            *self.resources_collected.entry(cell).or_insert(0) += 1;
        }
    }

    pub fn maybe_create_robot(&mut self) -> Option<Robot> {
        let energy = self.resources_collected.get(&Cell::Energy).copied().unwrap_or(0);
        let mineral = self.resources_collected.get(&Cell::Mineral).copied().unwrap_or(0);
        let science = self.resources_collected.get(&Cell::Science).copied().unwrap_or(0);

        // Check for each robot type in order of priority
        if energy >= 5 {
            *self.resources_collected.entry(Cell::Energy).or_insert(0) -= 5;
            self.robots_created += 1;
            println!("Station created a new Explorer robot!");
            Some(Robot::new(self.x, self.y, Direction::North, RobotRole::Explorer))
        } else if mineral >= 5 {
            *self.resources_collected.entry(Cell::Mineral).or_insert(0) -= 5;
            self.robots_created += 1;
            println!("Station created a new Collector robot!");
            Some(Robot::new(self.x, self.y, Direction::North, RobotRole::Collector))
        } else if science >= 5 {
            *self.resources_collected.entry(Cell::Science).or_insert(0) -= 5;
            self.robots_created += 1;
            println!("Station created a new Scientist robot!");
            Some(Robot::new(self.x, self.y, Direction::North, RobotRole::Scientist))
        } else {
            None
        }
    }
}
