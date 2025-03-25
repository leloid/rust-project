use crate::map::{Cell, Map};
use crate::robot::{Robot, RobotRole};
use std::collections::HashMap;
use crate::robot::Direction;

#[derive(Debug)]
pub struct Station {
    pub x: usize,
    pub y: usize,
    pub discovered: HashMap<(usize, usize), Cell>, // fusion des cartes
    pub resources_collected: u32,
    pub robots_created: u32,
}

impl Station {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            discovered: HashMap::new(),
            resources_collected: 0,
            robots_created: 3, // les 3 de départ
        }
    }

    pub fn receive_data(&mut self, data: Vec<((usize, usize), Cell)>) {
        for (pos, cell) in data {
            self.discovered.insert(pos, cell);
        }
    }

    pub fn receive_resources(&mut self, amount: u32) {
        self.resources_collected += amount;
    }

    pub fn maybe_create_robot(&mut self) -> Option<Robot> {
        if self.resources_collected >= 5 {
            self.resources_collected -= 5;
            self.robots_created += 1;
            println!("🚀 Station a créé un nouveau robot !");
            Some(Robot::new(self.x, self.y, Direction::North, RobotRole::Explorer))
        } else {
            None
        }
    }
}
