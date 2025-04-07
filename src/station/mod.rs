use crate::map::Cell;
use crate::robot::{Robot, RobotRole, Direction};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Station {
    pub x: usize,
    pub y: usize,
    pub discovered: HashMap<(usize, usize), Cell>, // fusion des cartes
    pub resources_collected: HashMap<Cell, u32>, 
    pub robots_created: u32,
    pub scientific_discoveries: u32, 
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

    pub fn receive_data(&mut self, data: Vec<((usize, usize), Cell)>) {
        for (pos, cell) in data {
            if cell == Cell::Science && !self.discovered.contains_key(&pos) {
                self.scientific_discoveries += 1;
            }
            self.discovered.insert(pos, cell);
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
