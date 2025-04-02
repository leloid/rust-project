use crate::map::{Map, Cell};
use rand::Rng;
use std::collections::{HashSet, VecDeque};

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
    pub collected: Vec<Cell>, 
    pub returning_to_station: bool, 
}

impl Robot {
    pub fn new(x: usize, y: usize, direction: Direction, role: RobotRole) -> Self {
        Self {
            x,
            y,
            direction,
            role,
            discovered: Vec::new(),
            collected: Vec::new(), 
            returning_to_station: false,
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
            _ => (self.x, self.y),
        };

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
    fn find_closest_resource(&self) -> Option<(usize, usize)> {
        self.discovered
            .iter()
            .filter(|(_, cell)| matches!(cell, Cell::Energy | Cell::Mineral))
            .map(|((x, y), _)| (*x, *y))
            .min_by_key(|(x, y)| self.x.abs_diff(*x) + self.y.abs_diff(*y))
    }
    
    pub fn act(&mut self, map: &mut Map, station_x: usize, station_y: usize) { 
        match self.role {
            RobotRole::Explorer => {
                self.vision(map, 2); 
                self.move_smart_towards_unknown(map); 
            }
            RobotRole::Collector => {
                //Collect dès que possible 
                self.collect_resource(map); 

                if self.returning_to_station {
                    self.move_dijkstra_to(station_x, station_y, map);
                    self.collect_resource(map); 
                    return;
                }
    
                // S'il a atteint le seuil de ressources, retourne
                if self.collected.len() >= 3 {
                    self.returning_to_station = true;
                    self.move_dijkstra_to(station_x, station_y, map);
                    return;
                }
    
                // Sinon cherche des ressources découvertes
                if let Some((rx, ry)) = self.find_closest_resource() {
                    self.move_dijkstra_to(rx, ry, map);
                    self.collect_resource(map); // collecte si déjà dessus
                } else {
                    // Fallback aléatoire
                    self.move_random(map);
                }            
            }
            RobotRole::Scientist => {
                if self.returning_to_station {
                    self.move_dijkstra_to(station_x, station_y, map);
                } else {
                    self.scan(map);
                    self.move_random(map);
                }
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
            let collected = *cell; 
            println!("🧺 Robot collecte à ({}, {}) : {:?}", self.x, self.y, collected);
            *cell = Cell::Empty;
            self.collected.push(collected);
        } else {
            self.move_random(map);
        }
    }

    fn scan(&mut self, map: &Map) {
        self.vision(map, 1);
        let mut new_discoveries = 0;

        for ((x, y), cell) in &self.discovered {
            if *cell == Cell::Science {
                println!("🔬 Découverte scientifique détectée à ({}, {}) !", x, y);
                new_discoveries += 1;
            }
        }

        if new_discoveries > 0 {
            println!("🧠 Robot Scientist a détecté {} science(s) et retourne à la station", new_discoveries);
            self.returning_to_station = true;
        }
    }

    fn move_smart_towards_unknown(&mut self, map: &Map) {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut came_from = vec![vec![None; map.width]; map.height];
    
        queue.push_back((self.x, self.y));
        visited.insert((self.x, self.y));
    
        let mut target: Option<(usize, usize)> = None;
    
        while let Some((cx, cy)) = queue.pop_front() {
            // Trouve une case inconnue atteignable
            if !self.discovered.iter().any(|&((x, y), _)| x == cx && y == cy) {
                target = Some((cx, cy));
                break;
            }
    
            for (dx, dy) in [(0isize, -1), (0, 1), (-1, 0), (1, 0)] {
                let nx = cx as isize + dx;
                let ny = cy as isize + dy;
    
                if nx >= 0 && ny >= 0 &&
                    (nx as usize) < map.width && (ny as usize) < map.height {
                    let ux = nx as usize;
                    let uy = ny as usize;
                    if !visited.contains(&(ux, uy)) && map.grid[uy][ux] != Cell::Obstacle {
                        visited.insert((ux, uy));
                        came_from[uy][ux] = Some((cx, cy));
                        queue.push_back((ux, uy));
                    }
                }
            }
        }
    
        // Reconstitue le chemin et avance vers la case inconnue
        if let Some((tx, ty)) = target {
            let mut path = vec![(tx, ty)];
            let mut current = came_from[ty][tx];
    
            while let Some((cx, cy)) = current {
                if (cx, cy) == (self.x, self.y) {
                    break;
                }
                path.push((cx, cy));
                current = came_from[cy][cx];
            }
    
            path.reverse();
    
            if let Some(&(nx, ny)) = path.get(0) {
                if nx > self.x {
                    self.direction = Direction::East;
                } else if nx < self.x {
                    self.direction = Direction::West;
                } else if ny > self.y {
                    self.direction = Direction::South;
                } else if ny < self.y {
                    self.direction = Direction::North;
                }
                self.move_forward(map);
                return;
            }
        }
    
        // Fallback
        self.move_random(map);
    }
    

    fn move_dijkstra_to(&mut self, goal_x: usize, goal_y: usize, map: &Map) {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut came_from = vec![vec![None; map.width]; map.height];

        queue.push_back((self.x, self.y));
        visited.insert((self.x, self.y));

        let dirs = [(0isize, -1isize), (0, 1), (-1, 0), (1, 0)];

        while let Some((cx, cy)) = queue.pop_front() {
            if (cx, cy) == (goal_x, goal_y) {
                break;
            }

            for (dx, dy) in dirs.iter() {
                let nx = cx as isize + dx;
                let ny = cy as isize + dy;

                if nx >= 0 && ny >= 0 &&
                    (nx as usize) < map.width && (ny as usize) < map.height {
                    let ux = nx as usize;
                    let uy = ny as usize;
                    if !visited.contains(&(ux, uy)) && map.grid[uy][ux] != Cell::Obstacle {
                        visited.insert((ux, uy));
                        came_from[uy][ux] = Some((cx, cy));
                        queue.push_back((ux, uy));
                    }
                }
            }
        }

        // Reconstruit le chemin depuis goal vers start
        let mut path = vec![(goal_x, goal_y)];
        let mut current = came_from[goal_y][goal_x];

        while let Some((cx, cy)) = current {
            if (cx, cy) == (self.x, self.y) {
                break;
            }
            path.push((cx, cy));
            current = came_from[cy][cx];
        }

        // Le chemin est de goal -> ... -> start, on inverse
        path.reverse();

        // Avance d’un pas si possible
        if let Some(&(next_x, next_y)) = path.get(0) {
            if next_x > self.x {
                self.direction = Direction::East;
            } else if next_x < self.x {
                self.direction = Direction::West;
            } else if next_y > self.y {
                self.direction = Direction::South;
            } else if next_y < self.y {
                self.direction = Direction::North;
            }

            self.move_forward(map);
        }

    }
}