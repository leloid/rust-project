use bevy::prelude::*;
use bevy::window::{Window, WindowPlugin};

use projet_essaim::config::{MAP_WIDTH, MAP_HEIGHT, SEED};
use projet_essaim::map::Map;
use projet_essaim::robot::{Robot, RobotRole, Direction};
use projet_essaim::station::Station;
use projet_essaim::resources::gui::{
    setup_simulation,
    tick_simulation,
    camera_zoom_system,
    camera_pan_system,
    SimulationData,
    SimulationTickTimer,
};
use bevy_pancam::PanCamPlugin;

fn main() {
    let map = Map::new(MAP_WIDTH, MAP_HEIGHT, SEED);

    let station_x = MAP_WIDTH / 2;
    let station_y = MAP_HEIGHT / 2;
    let station = Station::new(station_x, station_y);

    let robots = vec![
        Robot::new(station_x, station_y, Direction::East, RobotRole::Explorer),
        Robot::new(station_x - 2, station_y, Direction::East, RobotRole::Collector),
        Robot::new(station_x + 2, station_y, Direction::North, RobotRole::Scientist),
    ];

    App::new()
        .insert_resource(SimulationData {
            map,
            robots,
            station,
            station_x,
            station_y,
        })
        .insert_resource(SimulationTickTimer::new())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Projet Essaim 🌍".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(PanCamPlugin)
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, setup_simulation)
        .add_systems(Update, tick_simulation)
        .add_systems(Update, camera_zoom_system)
        .add_systems(Update, camera_pan_system)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        bevy_pancam::PanCam {
            grab_buttons: vec![MouseButton::Right],
            enabled: true,
            min_scale: 0.1,
            max_scale: 10.0,
            ..default()
        },
    ));
}
