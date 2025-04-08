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
}

#[derive(Debug)]
pub struct Robot {
    pub x: usize,
    pub y: usize,
    pub direction: Direction,
    pub role: RobotRole,
    pub discovered: Vec<((usize, usize), Cell)>,
}

impl Robot {
    pub fn new(x: usize, y: usize, direction: Direction, role: RobotRole) -> Self {
        Self {
            x,
            y,
            direction,
            role,
            discovered: Vec::new(),
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

    pub fn act(&mut self, map: &mut Map, _station_x: usize, _station_y: usize, _station: &crate::station::Station) { 
        match self.role {
            RobotRole::Explorer => {
                self.vision(map, 2); 
                self.move_smart_towards_unknown(map); 
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
}