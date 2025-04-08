use crate::map::{Map, Cell};
use rand::Rng;
use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RobotRole {
    Explorer,
    Collector,
}

#[derive(Debug, Clone)]
pub struct Robot {
    pub x: usize,
    pub y: usize,
    pub direction: Direction,
    pub role: RobotRole,
    pub discovered: Vec<((usize, usize), Cell)>,
    pub collected: Vec<Cell>,  // Resources collected by the collector
    pub target_resource: Option<Cell>,  // Current target resource type
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
            target_resource: None,
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

    pub fn act(&mut self, map: &mut Map, station_x: usize, station_y: usize, station: &mut crate::station::Station) {
        // Update vision for all robots
        self.vision(map, 2);
        
        match self.role {
            RobotRole::Explorer => {
                self.move_smart_towards_unknown(map);
            }
            RobotRole::Collector => {
                // If we have any resources, return to station
                if !self.collected.is_empty() {
                    // If we're at the station, deposit resources
                    if self.x == station_x && self.y == station_y {
                        station.receive_resources(self.collected.drain(..).collect());
                    } else {
                        // Move towards the station
                        self.move_dijkstra_to(map, station_x, station_y);
                    }
                } else {
                    // Find and collect resources
                    self.find_and_collect_resources(map, station);
                }
            }
        }
    }

    fn find_and_collect_resources(&mut self, map: &mut Map, station: &mut crate::station::Station) {
        // First check if we're standing on a resource
        let current_cell = map.grid[self.y][self.x];
        if (current_cell == Cell::Mineral || current_cell == Cell::Energy) && self.target_resource.is_none() {
            self.collected.push(current_cell);
            map.grid[self.y][self.x] = Cell::Empty;
            // Don't return, let act() handle the movement
        }

        // If we don't have a target resource, find one
        if self.target_resource.is_none() {
            self.target_resource = self.find_nearest_resource(map, station);
        }

        if let Some(target_type) = self.target_resource {
            // Move towards the target resource
            if let Some((target_x, target_y)) = self.find_resource_position(map, target_type) {
                self.move_dijkstra_to(map, target_x, target_y);
                
                // If we reached the resource, collect it
                if self.x == target_x && self.y == target_y {
                    self.collected.push(target_type);
                    map.grid[self.y][self.x] = Cell::Empty;  // Remove resource from map
                    self.target_resource = None;  // Reset target
                }
            } else {
                // If we can't find the target resource, reset target
                self.target_resource = None;
            }
        } else {
            // If no resources found, explore
            self.move_smart_towards_unknown(map);
        }
    }

    fn find_nearest_resource(&self, map: &Map, station: &mut crate::station::Station) -> Option<Cell> {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back((self.x, self.y));
        visited.insert((self.x, self.y));

        while let Some((x, y)) = queue.pop_front() {
            let cell = map.grid[y][x];
            if cell == Cell::Mineral || cell == Cell::Energy {
                // Check if this resource is already being targeted by another collector
                let mut is_targeted = false;
                for (pos, _) in &station.discovered {
                    if *pos == (x, y) {
                        is_targeted = true;
                        break;
                    }
                }
                if !is_targeted {
                    // Mark this resource as targeted
                    station.discovered.insert((x, y), cell);
                    return Some(cell);
                }
            }

            // Add neighbors to queue
            for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx >= 0 && ny >= 0 && nx < map.width as isize && ny < map.height as isize {
                    let pos = (nx as usize, ny as usize);
                    if !visited.contains(&pos) && map.grid[pos.1][pos.0] != Cell::Obstacle {
                        queue.push_back(pos);
                        visited.insert(pos);
                    }
                }
            }
        }
        None
    }

    fn find_resource_position(&self, map: &Map, target_type: Cell) -> Option<(usize, usize)> {
        for y in 0..map.height {
            for x in 0..map.width {
                if map.grid[y][x] == target_type {
                    return Some((x, y));
                }
            }
        }
        None
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

    fn move_dijkstra_to(&mut self, map: &mut Map, target_x: usize, target_y: usize) {
        println!("Moving from ({}, {}) to ({}, {})", self.x, self.y, target_x, target_y);
        
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut came_from = vec![vec![None; map.width]; map.height];

        queue.push_back((self.x, self.y));
        visited.insert((self.x, self.y));

        while let Some((cx, cy)) = queue.pop_front() {
            if (cx, cy) == (target_x, target_y) {
                let mut path = vec![(cx, cy)];
                let mut current = came_from[cy][cx];

                while let Some((nx, ny)) = current {
                    path.push((nx, ny));
                    current = came_from[ny][nx];
                }

                path.reverse();

                // Find the first step in the path that's not our current position
                if let Some(&(nx, ny)) = path.iter().find(|&&(x, y)| (x, y) != (self.x, self.y)) {
                    println!("Next step: ({}, {})", nx, ny);
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

            for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
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
        
        // If we can't find a path, try to move in the general direction
        println!("No path found, moving towards target");
        if target_x > self.x {
            self.direction = Direction::East;
        } else if target_x < self.x {
            self.direction = Direction::West;
        } else if target_y > self.y {
            self.direction = Direction::South;
        } else if target_y < self.y {
            self.direction = Direction::North;
        }
        self.move_forward(map);
    }
}