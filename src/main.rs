mod map;
mod robot;
mod station;
mod resources;
mod config;
use bevy::prelude::*;
use bevy::window::{Window, WindowPlugin, WindowPosition, MonitorSelection};

use resources::gui::{setup_simulation, SimulationData};
use map::Map;
use robot::{Robot, RobotRole, Direction};

fn main() {
    let seed = 42;
    let map = Map::new(1200, 15, seed);

    let robots = vec![
        Robot::new(5, 3, Direction::East, RobotRole::Explorer),
        Robot::new(5, 1, Direction::North, RobotRole::Explorer),
        Robot::new(7, 2, Direction::North, RobotRole::Scientist),
        Robot::new(1, 3, Direction::East, RobotRole::Collector),
    ];

    let station_x = 5;
    let station_y = 3;

    App::new()
        .insert_resource(SimulationData {
            map,
            robots,
            station_x,
            station_y,
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Projet Essaim 🌍".to_string(),
                resolution: (1200.0, 800.0).into(),
                position: WindowPosition::Centered(MonitorSelection::Primary),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup_simulation)
        .run();
}
