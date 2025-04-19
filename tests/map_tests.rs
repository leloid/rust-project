use projet_essaim::map::{Map, Cell};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_creation() {
        let map = Map::new(10, 10, 42);
        assert_eq!(map.width, 10);
        assert_eq!(map.height, 10);
        assert_eq!(map.grid.len(), 10);
        assert_eq!(map.grid[0].len(), 10);
    }

    #[test]
    fn test_map_cell_types() {
        let map = Map::new(5, 5, 42);
        // Vérifier que la grille contient bien des Cell
        for row in &map.grid {
            for cell in row {
                match cell {
                    Cell::Empty | Cell::Obstacle | Cell::Energy | Cell::Mineral | Cell::Science => (),
                }
            }
        }
    }
} 