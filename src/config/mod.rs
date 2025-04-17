pub const MAP_WIDTH: usize = 50;  // Increased from 20
pub const MAP_HEIGHT: usize = 50; // Increased from 20
pub const SEED: u64 = 42;
pub const FOG_OF_WAR: bool = true; // Set to false to see the entire map

// Robot creation costs
pub const EXPLORER_COST: usize = 10;  // Energy cost to create an explorer
pub const COLLECTOR_COST: usize = 10; // Mineral cost to create a collector
pub const SCIENTIST_COST: usize = 10; // Science cost to create a scientist