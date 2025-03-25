use crate::map::{Map, Cell};
use rand::Rng;


#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
pub enum RobotRole {
    Explorer,
    Collector,
    Scientist,
}

#[derive(Debug)]
pub struct Robot {
    pub x: usize,
    pub y: usize,
    pub direction: Direction,
    pub role: RobotRole,
    pub discovered: Vec<((usize, usize), Cell)>,
    pub collected_energy: u32,
}


impl Robot {
    
    pub fn new(x: usize, y: usize, direction: Direction, role: RobotRole) -> Self {
        Self {
            x,
            y,
            direction,
            role,
            discovered: Vec::new(),
            collected_energy: 0,
        }        
    }


    pub fn turn_left(&mut self) {
        self.direction = match self.direction {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        };
    }

    pub fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
    }

    pub fn move_forward(&mut self, map: &Map) {
        let (new_x, new_y) = match self.direction {
            Direction::North if self.y > 0 => (self.x, self.y - 1),
            Direction::South if self.y < map.height - 1 => (self.x, self.y + 1),
            Direction::West if self.x > 0 => (self.x - 1, self.y),
            Direction::East if self.x < map.width - 1 => (self.x + 1, self.y),
            _ => (self.x, self.y), // mur ou bord
        };

        // Déplacement si la case n’est pas un obstacle
        if map.grid[new_y][new_x] != Cell::Obstacle {
            self.x = new_x;
            self.y = new_y;
        }
    }

    pub fn vision(&mut self, map: &Map, range: usize) {
        let min_x = self.x.saturating_sub(range);
        let max_x = usize::min(self.x + range, map.width - 1);
        let min_y = self.y.saturating_sub(range);
        let max_y = usize::min(self.y + range, map.height - 1);
    
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let cell = map.grid[y][x];
                self.discovered.push(((x, y), cell));
            }
        }
    }
    


    /////////////////////
    pub fn act(&mut self, map: &mut Map) {
        match self.role {
            RobotRole::Explorer => {
                self.move_random(map);
            }
            RobotRole::Collector => {
                self.collect_resource(map);
            }
            RobotRole::Scientist => {
                self.scan(map);
            }
        }
    }

    fn move_random(&mut self, map: &Map) {
        let mut rng = rand::thread_rng();
        let dir = rng.gen_range(0..4);
        self.direction = match dir {
            0 => Direction::North,
            1 => Direction::South,
            2 => Direction::East,
            _ => Direction::West,
        };
        self.move_forward(map);
    }

    fn collect_resource(&mut self, map: &mut Map) {
        let cell = &mut map.grid[self.y][self.x];
        if *cell == Cell::Energy || *cell == Cell::Mineral {
            println!("🧺 Robot collecte à ({}, {}) : {:?}", self.x, self.y, cell);
            *cell = Cell::Empty;
            self.collected_energy += 1;
        }
         else {
            self.move_random(map);
        }
    }

    fn scan(&mut self, map: &Map) {
        self.vision(map, 1);
        for ((x, y), cell) in &self.discovered {
            if *cell == Cell::Science {
                println!("🔬 Découverte scientifique détectée à ({}, {}) !", x, y);
            }
        }
    }
    
}
