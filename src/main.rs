mod station;
mod resources;
mod config;
mod map;
use map::Map;

mod robot;
use robot::{Robot, Direction, RobotRole};

use station::Station;

fn main() {
    let seed = 42;
    let mut map = Map::new(30, 10, seed);

    // Création de la station à une position fixe (par exemple au centre)
    let mut station = Station::new(5, 3);

    // Création des robots initiaux
    let mut robots = vec![
        Robot::new(5, 3, Direction::East, RobotRole::Explorer),
        Robot::new(5, 3, Direction::South, RobotRole::Collector),
        // Robot::new(5, 3, Direction::North, RobotRole::Scientist),
        // Robot::new(8, 3, Direction::North, RobotRole::Scientist),
        Robot::new(6, 2, Direction::East, RobotRole::Scientist),

    ];

    println!("🎮 Carte initiale avec robots");
    // for robot in &robots {
    //     map.display_with_robot(robot);
    // }
    map.display_with_entities(&robots, station.x, station.y);


    // Simulation de plusieurs ticks
    for tick in 1..=10 {
        println!("=====================");
        println!("\n🚀 Tick {} : chaque robot agit !", tick);
        println!("📦 Ressources collectées jusqu'ici :");
        for (cell, count) in &station.resources_collected {
            println!("   - {:?} : {}", cell, count);
        }
        
        for robot in &mut robots {
            robot.act(&mut map, station.x, station.y);

            // Le robot revient à la station pour synchroniser ses données
            if robot.x == station.x && robot.y == station.y {
                println!("🔁 Robot est de retour à la station !");
                robot.returning_to_station = false;
                station.receive_data(robot.discovered.drain(..).collect());
                station.receive_resources(robot.collected.drain(..).collect());
            }
            
        }

        // La station peut créer un nouveau robot si elle a assez de ressources
        if let Some(new_robot) = station.maybe_create_robot() {
            robots.push(new_robot);
        }

        // Affichage de la carte après chaque tick
        println!("\n🗺️ Carte après Tick {} :", tick);
        map.display_with_entities(&robots, station.x, station.y);

    }

    // Affichage des infos finales
    println!("\n📡 Mission terminée !");
    println!("📚 Zones découvertes par la station : {}", station.discovered.len());
    println!("🔬 Découvertes scientifiques : {}", station.scientific_discoveries); // <- AJOUTÉ
    println!("⚡ Ressources collectées :");
    for (cell, count) in &station.resources_collected {
        println!("   - {:?} : {}", cell, count);
    }
        println!("🤖 Robots créés : {}", station.robots_created);
}
