use noise::{NoiseFn, Perlin};
use rand::{SeedableRng, rngs::StdRng, Rng};
use crate::robot::Robot;
use crate::robot::RobotRole;
use crate::station::Station;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cell {
    Empty,
    Obstacle,
    Energy,
    Mineral,
    Science,
}


#[derive(Debug, Clone)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Vec<Cell>>,
}

impl Map {
    pub fn new(width: usize, height: usize, seed: u64) -> Self {
        let perlin = Perlin::new(seed as u32);
        let mut grid = vec![vec![Cell::Empty; width]; height];
        let scale = 0.1;

        for y in 0..height {
            for x in 0..width {
                let noise_val = perlin.get([x as f64 * scale, y as f64 * scale, seed as f64]);
                if noise_val > 0.4 {
                    grid[y][x] = Cell::Obstacle;
                }
            }
        }

        let mut rng = StdRng::seed_from_u64(seed);

        // Place resources
        Map::place_random(&mut grid, Cell::Energy, 20, &mut rng);
        Map::place_random(&mut grid, Cell::Mineral, 20, &mut rng);
        Map::place_random(&mut grid, Cell::Science, 10, &mut rng);

        Self { width, height, grid }
    }

    pub fn place_station(&mut self, x: usize, y: usize) {
        // Ensure the position is within bounds
        if x < self.width && y < self.height {
            // Clear the station's position
            self.grid[y][x] = Cell::Empty;
        }
    }

    fn place_random(grid: &mut Vec<Vec<Cell>>, kind: Cell, count: usize, rng: &mut StdRng) {
        let height = grid.len();
        let width = grid[0].len();
        let mut placed = 0;

        while placed < count {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);

            if grid[y][x] == Cell::Empty {
                grid[y][x] = kind;
                placed += 1;
            }
        }
    }

    pub fn display(&self) {
        for row in &self.grid {
            for cell in row {
                let symbol = match cell {
                    Cell::Empty => " E ",
                    Cell::Obstacle => " O ",
                    Cell::Energy => " P ",
                    Cell::Mineral => " M ",
                    Cell::Science => " S ",
                };
                print!("{}", symbol);
            }
            println!();
        }
    }

    pub fn display_with_robot(&self, robot: &Robot) {
        for y in 0..self.height {
            for x in 0..self.width {
                if robot.x == x && robot.y == y {
                    print!(" R ");
                } else {
                    let symbol = match self.grid[y][x] {
                        Cell::Empty => " E ",
                        Cell::Obstacle => " O ",
                        Cell::Energy => " P ",
                        Cell::Mineral => " M ",
                        Cell::Science => " S ",
                    };
                    print!("{}", symbol);
                }
            }
            println!();
        }
    }

    pub fn display_with_entities(&self, robots: &[Robot], station_x: usize, station_y: usize) {
        for y in 0..self.height {
            for x in 0..self.width {
                // Détermine le symbole à afficher selon priorité
                let symbol = if robots.iter().any(|r| r.x == x && r.y == y) {
                    "\x1b[31m R \x1b[0m " 
                } else if x == station_x && y == station_y {
                    "\x1b[34m H \x1b[0m " 
                } else {
                    match self.grid[y][x] {
                        Cell::Empty => " E ",
                        Cell::Obstacle => "\x1b[90m O \x1b[0m ",
                        Cell::Energy => "\x1b[33m P \x1b[0m ",    
                        Cell::Mineral => "\x1b[35m M \x1b[0m ",   
                        Cell::Science => "\x1b[36m S \x1b[0m ",   
                    }
                };
    
                // Affiche le symbole en largeur fixe (4 espaces pour l'alignement parfait)
                print!("{:<4}", symbol);
            }
            println!();
        }
    }

    pub fn display_with_fog(&self, robots: &[Robot], station_x: usize, station_y: usize, station: &Station) {
        let mut visible_cells = HashSet::new();
        
        // Add station's initial vision
        visible_cells.insert((station_x, station_y));

        // Add robots' initial positions
        for robot in robots {
            visible_cells.insert((robot.x, robot.y));
        }

        // Add discovered cells from the station
        for (&(x, y), _) in &station.discovered {
            visible_cells.insert((x, y));
        }
    
        for y in 0..self.height {
            for x in 0..self.width {
                let symbol = if robots.iter().any(|r| r.x == x && r.y == y) {
                    "\x1b[31m R \x1b[0m " // Robot
                } else if x == station_x && y == station_y {
                    "\x1b[34m H \x1b[0m " // Station
                } else if visible_cells.contains(&(x, y)) {
                    match self.grid[y][x] {
                        Cell::Empty => " E ",
                        Cell::Obstacle => "\x1b[90m O \x1b[0m ",
                        Cell::Energy => "\x1b[33m P \x1b[0m ",
                        Cell::Mineral => "\x1b[35m M \x1b[0m ",
                        Cell::Science => "\x1b[36m S \x1b[0m ",
                    }
                } else {
                    " ? " // Zone non explorée
                };
                
                print!("{:<4}", symbol);
            }
            println!();
        }
    }
    
}
