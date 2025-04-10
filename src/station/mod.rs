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
        }
    }



    pub fn receive_resources(&mut self, collected_cells: Vec<Cell>) {
        for cell in collected_cells {
            *self.resources_collected.entry(cell).or_insert(0) += 1;
        }
    }

    pub fn maybe_create_robot(&mut self) -> Option<Robot> {
        let energy = self.resources_collected.get(&Cell::Energy).copied().unwrap_or(0);
        if energy >= 5 {
            *self.resources_collected.entry(Cell::Energy).or_insert(0) -= 5;
            self.robots_created += 1;
            println!("🚀 Station a créé un nouveau robot !");
            Some(Robot::new(self.x, self.y, Direction::North, RobotRole::Explorer))
        } else {
            None
        }
    }
}
