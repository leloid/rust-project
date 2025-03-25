use noise::{NoiseFn, Perlin};
use rand::{SeedableRng, rngs::StdRng, Rng};
use crate::robot::Robot;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cell {
    Empty,
    Obstacle,
    Energy,
    Mineral,
    Science,
}


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
                    Cell::Empty => " . ",
                    Cell::Obstacle => "███",
                    Cell::Energy => " ⚡ ",
                    Cell::Mineral => " ⛏️ ",
                    Cell::Science => " 🔬",
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
                print!(" 🤖 ");
            } else {
                let symbol = match self.grid[y][x] {
                    Cell::Empty => " . ",
                    Cell::Obstacle => "███",
                    Cell::Energy => " ⚡ ",
                    Cell::Mineral => " ⛏️ ",
                    Cell::Science => " 🔬",
                };
                print!("{}", symbol);
            }
        }
        println!();
    }
}

}
