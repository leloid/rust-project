mod robot;
mod station;
mod resources;
mod config;
mod map;
use map::Map;

fn main() {
    let seed = 42;
    let map = Map::new(30, 15, seed);

    println!("🧭 Carte générée avec seed = {seed}");
    map.display();
}
