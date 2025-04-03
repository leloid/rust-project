use bevy::prelude::*;
use bevy::window::{Window, WindowPlugin, WindowPosition, MonitorSelection};

use projet_essaim::config::{MAP_WIDTH, MAP_HEIGHT, SEED};
use projet_essaim::map::Map;
use projet_essaim::robot::{Robot, RobotRole, Direction};
use projet_essaim::resources::gui::{
    setup_simulation,
    tick_simulation,
    camera_zoom_system,
    camera_pan_system,
    SimulationData,
};

fn main() {
    let map = Map::new(MAP_WIDTH, MAP_HEIGHT, SEED);

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
        .add_systems(Update, tick_simulation)
        .add_systems(Update, camera_zoom_system)
        .add_systems(Update, camera_pan_system)
        .run();
}
