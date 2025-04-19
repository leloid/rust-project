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
    map.place_station(station_x, station_y);  // Ensure station position is empty
    let mut station = Station::new(station_x, station_y);

    // Création des robots
    let mut robots = vec![
        Robot::new(5, 3, Direction::East, RobotRole::Explorer),
        Robot::new(1, 3, Direction::East, RobotRole::Scientist),
        Robot::new(1, 1, Direction::East, RobotRole::Collector),
    ];

    println!("Carte initiale avec brouillard de guerre");
    // map.display_with_entities(&robots, station_x, station_y);
    // map.display_with_entities(&robots, station_x, station_y);
    map.display_with_fog(&robots, station_x, station_y,&station);

    // Simulation de plusieurs ticks
    for tick in 1..=50 {
        println!("=====================");
        println!("\nTick {} : exploration en cours !", tick);

        for (i, robot) in robots.iter_mut().enumerate() {
            println!("Robot #{} ({:?}) en position ({}, {})", i, robot.role, robot.x, robot.y);
            if robot.role == RobotRole::Collector {
                println!("   Ressources collectées : {:?}", robot.collected);
                if let Some(target) = robot.target_resource {
                    println!("   Cible actuelle : {:?}", target);
                }
            }
            robot.act(&mut map, station_x, station_y, &mut station);
        }

        // Affichage de la carte après chaque tick
        println!("\nCarte après Tick {} :", tick);
        map.display_with_fog(&robots, station_x, station_y,&station);
        
        // Afficher les ressources restantes
        let mut minerals = 0;
        let mut energy = 0;
        for row in &map.grid {
            for cell in row {
                match cell {
                    Cell::Mineral => minerals += 1,
                    Cell::Energy => energy += 1,
                    _ => {}
                }
            }
        }
        println!("   Minéraux restants : {}", minerals);
        println!("   Énergie restante : {}", energy);
    }

    // Affichage des infos finales station + robots
    println!("\nExploration terminée !");
    for (i, robot) in robots.iter().enumerate() {
        println!("Robot #{} ({:?})", i, robot.role);
        println!("   Position finale : ({}, {})", robot.x, robot.y);
        if robot.role == RobotRole::Collector {
            println!("   Ressources collectées : {:?}", robot.collected);
        }
    }

    // Infos fusionnées à la station
    println!("\nStation - Données fusionnées :");
    println!("   Zones explorées (total unique) : {}", station.discovered.len());
    println!("   Ressources collectées : {:?}", station.resources_collected);
    println!("   Robots créés au total : {}", station.robots_created);
}
