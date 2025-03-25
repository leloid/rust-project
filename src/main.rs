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
    let mut map = Map::new(30, 15, seed);

    // Création de la station à une position fixe (par exemple au centre)
    let mut station = Station::new(5, 5);

    // Création des robots initiaux
    let mut robots = vec![
        Robot::new(5, 5, Direction::East, RobotRole::Explorer),
        Robot::new(5, 5, Direction::South, RobotRole::Collector),
        Robot::new(5, 5, Direction::North, RobotRole::Scientist),
    ];

    println!("🎮 Carte initiale avec robots");
    for robot in &robots {
        map.display_with_robot(robot);
    }

    // Simulation de plusieurs ticks
    for tick in 1..=5 {
        println!("\n🚀 Tick {} : chaque robot agit !", tick);

        for robot in &mut robots {
            robot.act(&mut map);

            // Le robot revient à la station pour synchroniser ses données
            if robot.x == station.x && robot.y == station.y {
                println!("🔁 Robot est de retour à la station !");
                station.receive_data(robot.discovered.drain(..).collect());
                station.receive_resources(robot.collected_energy);
                robot.collected_energy = 0;
            }
        }

        // La station peut créer un nouveau robot si elle a assez de ressources
        if let Some(new_robot) = station.maybe_create_robot() {
            robots.push(new_robot);
        }

        // Affichage de la carte après chaque tick
        println!("\n🗺️ Carte après Tick {} :", tick);
        for robot in &robots {
            map.display_with_robot(robot);
        }
    }

    // Affichage des infos finales
    println!("\n📡 Mission terminée !");
    println!("📚 Zones découvertes par la station : {}", station.discovered.len());
    println!("⚡ Ressources collectées : {}", station.resources_collected);
    println!("🤖 Robots créés : {}", station.robots_created);
}
