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
    Scientist,  // New role for Scientist robot
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
    pub current_path: Vec<(usize, usize)>,  // Current path to follow
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
            current_path: Vec::new(),
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

    pub fn vision(&mut self, map: &Map, range: usize, station: &mut crate::station::Station) {
        let min_x = self.x.saturating_sub(range);
        let max_x = usize::min(self.x + range, map.width - 1);
        let min_y = self.y.saturating_sub(range);
        let max_y = usize::min(self.y + range, map.height - 1);

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let cell = map.grid[y][x];
                // Update the robot's discovered list
                if !self.discovered.iter().any(|&((dx, dy), _)| dx == x && dy == y) {
                    self.discovered.push(((x, y), cell));
                }
                // Update the station's discovered map
                station.discovered.entry((x, y)).or_insert(cell);
            }
        }
    }

    pub fn act(&mut self, map: &mut Map, station_x: usize, station_y: usize, station: &mut crate::station::Station) {
        // Update vision for all robots
        self.vision(map, 2, station);
        
        match self.role {
            RobotRole::Explorer => {
                // Check if we're at the station to deposit discoveries
                if self.x == station_x && self.y == station_y {
                    // We're at the station, we've already shared discoveries in vision()
                    // Now move away from station
                    self.move_smart_towards_unknown_with_others(map, station.get_explorer_positions());
                } else {
                    // Normal exploration - use the new spreading algorithm
                    self.move_smart_towards_unknown_with_others(map, station.get_explorer_positions());
                }
            }
            RobotRole::Collector => {
                // Check if we're on a resource and collect it
                let current_cell = map.grid[self.y][self.x];
                if (current_cell == Cell::Mineral || current_cell == Cell::Energy) && self.collected.len() < 2 {
                    self.collected.push(current_cell);
                    map.grid[self.y][self.x] = Cell::Empty;
                    println!("🤖 Collector collected a resource! Total collected: {}", self.collected.len());
                }

                // If we have 2 resources, return to station
                if self.collected.len() >= 2 {
                    if self.x == station_x && self.y == station_y {
                        println!("🤖 Collector depositing {} resources at station", self.collected.len());
                        station.receive_resources(self.collected.drain(..).collect());
                    } else {
                        self.move_dijkstra_to(map, station_x, station_y);
                    }
                } else {
                    // Look for nearest resource
                    if let Some((target_x, target_y)) = self.find_nearest_resource_position(map) {
                        self.move_dijkstra_to(map, target_x, target_y);
                    } else {
                        // If no resources found, explore
                        self.move_smart_towards_unknown(map);
                    }
                }
            }
            RobotRole::Scientist => {
                // Check if we're on a Science resource and collect it
                let current_cell = map.grid[self.y][self.x];
                if current_cell == Cell::Science && self.collected.len() < 1 {
                    self.collected.push(current_cell);
                    map.grid[self.y][self.x] = Cell::Empty;
                    println!("🤖 Scientist collected a science resource! Total collected: {}", self.collected.len());
                }

                // If we have a science resource, return to station
                if self.collected.len() >= 1 {
                    if self.x == station_x && self.y == station_y {
                        println!("🤖 Scientist depositing {} science resources at station", self.collected.len());
                        station.receive_resources(self.collected.drain(..).collect());
                    } else {
                        self.move_dijkstra_to(map, station_x, station_y);
                    }
                } else {
                    // Look for nearest science resource
                    if let Some((target_x, target_y)) = self.find_nearest_scientist_position(map) {
                        self.move_dijkstra_to(map, target_x, target_y);
                    } else {
                        // If no science resources found, move randomly
                        self.move_random(map);
                    }
                }
            }
        }
    }

    fn find_nearest_resource_position(&self, map: &Map) -> Option<(usize, usize)> {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back((self.x, self.y));
        visited.insert((self.x, self.y));

        while let Some((x, y)) = queue.pop_front() {
            let cell = map.grid[y][x];
            if cell == Cell::Mineral || cell == Cell::Energy {
                return Some((x, y));
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

    fn find_nearest_resource(&self, map: &Map, station: &mut crate::station::Station) -> Option<Cell> {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back((self.x, self.y));
        visited.insert((self.x, self.y));

        while let Some((x, y)) = queue.pop_front() {
            let cell = map.grid[y][x];
            if cell == Cell::Mineral || cell == Cell::Energy {
                // Check if this resource is already being targeted by another collector
                if !station.discovered.contains_key(&(x, y)) {
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
        
        // If we already have a path, follow it
        if !self.current_path.is_empty() {
            if let Some(&(nx, ny)) = self.current_path.first() {
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
                if self.x == nx && self.y == ny {
                    self.current_path.remove(0);
                }
                return;
            }
        }

        // Calculate new path
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
                // Remove our current position from the path
                if !path.is_empty() && path[0] == (self.x, self.y) {
                    path.remove(0);
                }
                self.current_path = path;
                break;
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
        if self.current_path.is_empty() {
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

    fn find_nearest_scientist_position(&self, map: &Map) -> Option<(usize, usize)> {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back((self.x, self.y));
        visited.insert((self.x, self.y));

        while let Some((x, y)) = queue.pop_front() {
            let cell = map.grid[y][x];
            if cell == Cell::Science {
                return Some((x, y));
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

    fn move_smart_towards_unknown_with_others(&mut self, map: &Map, other_explorers: &[(usize, usize)]) {
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        let mut visited = HashSet::new();
        let mut came_from = vec![vec![None; map.width]; map.height];
        
        // Create a cost map where cells near other explorers have higher costs
        let mut cost_map = vec![vec![1usize; map.width]; map.height];
        
        // Add higher costs to areas near other explorers to encourage spreading out
        for &(ex, ey) in other_explorers {
            // Skip if this is the current robot
            if ex == self.x && ey == self.y {
                continue;
            }
            
            // Define an influence radius (how far to avoid other explorers)
            let influence_radius = 8;
            
            // Add higher costs in a radius around other explorers
            for y in ey.saturating_sub(influence_radius)..=(ey + influence_radius).min(map.height - 1) {
                for x in ex.saturating_sub(influence_radius)..=(ex + influence_radius).min(map.width - 1) {
                    // Calculate Manhattan distance
                    let distance = (x as isize - ex as isize).abs() + (y as isize - ey as isize).abs();
                    
                    if distance < influence_radius as isize {
                        // Inverse relationship: closer = higher cost
                        let additional_cost = influence_radius as usize - distance as usize;
                        cost_map[y][x] += additional_cost * 3; // Multiply by a factor to increase the effect
                    }
                }
            }
        }
        
        // Start from the current position
        queue.push_back((self.x, self.y));
        visited.insert((self.x, self.y));
        
        // Keep track of the best target and its cost
        let mut best_target: Option<(usize, usize)> = None;
        let mut best_cost = usize::MAX;
        
        // BFS with cost consideration to find the best unknown cell
        while let Some((cx, cy)) = queue.pop_front() {
            // Check if this is an unknown cell
            if !self.discovered.iter().any(|&((x, y), _)| x == cx && y == cy) {
                // Calculate total cost to reach this cell
                let mut total_cost = 0;
                let mut current_pos = Some((cx, cy));
                
                while let Some((x, y)) = current_pos {
                    total_cost += cost_map[y][x];
                    current_pos = came_from[y][x];
                }
                
                // If this path has a lower cost than the current best, update it
                if total_cost < best_cost {
                    best_target = Some((cx, cy));
                    best_cost = total_cost;
                }
                
                // Don't break immediately; check all cells at the current BFS depth
                if queue.is_empty() || queue.front().map(|&(x, y)| 
                    !self.discovered.iter().any(|&((dx, dy), _)| dx == x && dy == y)
                ).unwrap_or(false) {
                    break;
                }
            }
            
            // Add neighbors to the queue
            for (dx, dy) in [(0isize, -1), (0, 1), (-1, 0), (1, 0)] {
                let nx = cx as isize + dx;
                let ny = cy as isize + dy;
                
                if nx >= 0 && ny >= 0 && (nx as usize) < map.width && (ny as usize) < map.height {
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
        
        // If we found a target, reconstruct the path and move
        if let Some((tx, ty)) = best_target {
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
        
        // Fallback to random movement if no path found
        self.move_random(map);
    }
}