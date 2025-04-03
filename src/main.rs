mod station;
mod resources;
mod config;
mod map;
use map::Map;
use map::Cell;

mod robot;
use robot::{Robot, Direction, RobotRole};
use std::collections::HashSet;
use config::{MAP_WIDTH, MAP_HEIGHT, SEED};

use station::Station;

fn main() {
    let seed = 42;
    let mut map = Map::new(MAP_WIDTH, MAP_HEIGHT, SEED);

    // Création de la station
    let station_x = 5;
    let station_y = 3;
    let mut station = Station::new(station_x, station_y);

    // Création des robots
    let mut robots = vec![
        Robot::new(5, 3, Direction::East, RobotRole::Explorer),
        Robot::new(5, 1, Direction::North, RobotRole::Explorer),
        Robot::new(7, 2, Direction::North, RobotRole::Explorer),
        Robot::new(1, 3, Direction::East, RobotRole::Explorer),
    ];

    println!("🎮 Carte initiale avec brouillard de guerre");
    map.display_with_fog(&robots, station_x, station_y);

    // Simulation de plusieurs ticks
    for tick in 1..=450 {
        println!("=====================");
        println!("\n🚀 Tick {} : exploration en cours !", tick);

        for (i, robot) in robots.iter_mut().enumerate() {
            println!("🤖 Robot #{} ({:?}) en position ({}, {})", i, robot.role, robot.x, robot.y);
            robot.act(&mut map, station_x, station_y);

            // S'il revient à la station
            if robot.x == station_x && robot.y == station_y {
                // Vérifie s'il a découvert de la science non encore validée
                let new_sciences: Vec<_> = robot
                    .discovered
                    .iter()
                    .filter(|(_, cell)| *cell == Cell::Science)
                    .cloned()
                    .collect();
            
                if !new_sciences.is_empty() {
                    println!("✅ Validation scientifique reçue à la station ! 🔬✨");
                }
            
                // Transfert des données à la station
                station.receive_data(robot.discovered.drain(..).collect());
                station.receive_resources(robot.collected.drain(..).collect());
            }
            
            if !robot.collected.is_empty() {
                println!("📦 Robot #{} a collecté :", i);
                for cell in &robot.collected {
                    println!("   - {:?}", cell);
                }
            }
            
        }

        // Affichage de la carte après chaque tick
        println!("\n🗺️ Carte après Tick {} :", tick);
        map.display_with_fog(&robots, station_x, station_y);
    }

    // ⚡ Synchronisation finale avec la station
    for robot in &mut robots {
        station.receive_data(robot.discovered.drain(..).collect());
        station.receive_resources(robot.collected.drain(..).collect());
    }

    // ✅ Affichage des infos finales station + robots
    println!("\n📡 Exploration terminée !");
    for (i, robot) in robots.iter().enumerate() {
        println!("📊 Robot #{} ({:?})", i, robot.role);
        println!("   📍 Position finale : ({}, {})", robot.x, robot.y);
    }

    // 📦 Infos fusionnées à la station
    println!("\n🏠 Station - Données fusionnées :");
    println!("   🧠 Découvertes scientifiques : {}", station.scientific_discoveries);
    println!("   🔍 Zones explorées (total unique) : {}", station.discovered.len());
    println!("   🔧 Ressources collectées :");
    for (cell, count) in &station.resources_collected {
        println!("      - {:?} : {}", cell, count);
    }
    println!("   🤖 Robots créés au total : {}", station.robots_created);

}
