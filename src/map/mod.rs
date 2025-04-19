pub mod cell;
pub use cell::Cell;
use noise::{NoiseFn, Perlin};
use rand::{SeedableRng, rngs::StdRng, Rng};
use crate::robot::Robot;
use crate::station::Station;
use std::collections::HashSet;


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

        let map_area = width * height;
        let energy_count = (map_area as f32 * 0.05) as usize;
        let mineral_count = (map_area as f32 * 0.05) as usize;
        let science_count = (map_area as f32 * 0.03) as usize;

        Map::place_random(&mut grid, Cell::Energy, energy_count, &mut rng);
        Map::place_random(&mut grid, Cell::Mineral, mineral_count, &mut rng);
        Map::place_random(&mut grid, Cell::Science, science_count, &mut rng);

        Self { width, height, grid }
    }

    pub fn place_station(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
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
                print!("{}", cell.to_symbol());
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
                    print!("{}", self.grid[y][x].to_symbol());
                }
            }
            println!();
        }
    }

    pub fn display_with_entities(&self, robots: &[Robot], station_x: usize, station_y: usize) {
        for y in 0..self.height {
            for x in 0..self.width {
                let symbol = if robots.iter().any(|r| r.x == x && r.y == y) {
                    "\x1b[31m R \x1b[0m "
                } else if x == station_x && y == station_y {
                    "\x1b[34m H \x1b[0m "
                } else {
                    self.grid[y][x].to_colored_symbol()
                };

                print!("{:<4}", symbol);
            }
            println!();
        }
    }

    pub fn display_with_fog(&self, robots: &[Robot], station_x: usize, station_y: usize, station: &Station) {
        let mut visible_cells = HashSet::new();

        visible_cells.insert((station_x, station_y));
        for robot in robots {
            visible_cells.insert((robot.x, robot.y));
        }
        for (&(x, y), _) in &station.discovered {
            visible_cells.insert((x, y));
        }

        for y in 0..self.height {
            for x in 0..self.width {
                let symbol = if robots.iter().any(|r| r.x == x && r.y == y) {
                    "\x1b[31m R \x1b[0m "
                } else if x == station_x && y == station_y {
                    "\x1b[34m H \x1b[0m "
                } else if visible_cells.contains(&(x, y)) {
                    self.grid[y][x].to_colored_symbol()
                } else {
                    " ? "
                };

                print!("{:<4}", symbol);
            }
            println!();
        }
    }
}
