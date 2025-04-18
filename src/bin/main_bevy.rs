use bevy::prelude::*;
use bevy::window::{Window, WindowPlugin};

use projet_essaim::config::{MAP_WIDTH, MAP_HEIGHT, SEED};
use projet_essaim::map::Map;
use projet_essaim::robot::{Robot, RobotRole, Direction};
use projet_essaim::station::Station;
use projet_essaim::resources::gui::{
    setup_simulation,
    tick_simulation,
    camera_pan_system,
    update_legend_counts,
    update_fog_of_war,
    update_tick_counter,
    update_window_title,
    handle_speed_keyboard,
    handle_play_pause_button,
    handle_speed_buttons,
    SimulationData,
    SimulationTickTimer,
    TickCounter,
    TickSpeedMultiplier,
    SimulationPaused,
    TILE_SIZE,
    update_speed_indicator,
};

fn main() {
    let map = Map::new(MAP_WIDTH, MAP_HEIGHT, SEED);

    let station_x = MAP_WIDTH / 2;
    let station_y = MAP_HEIGHT / 2;
    let station = Station::new(station_x, station_y);

    let robots = vec![
        Robot::new(station_x - 2, station_y, Direction::East, RobotRole::Explorer),
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
        .insert_resource(TickCounter::new())
        .insert_resource(TickSpeedMultiplier::new())
        .insert_resource(SimulationPaused::new())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Projet Essaim 🌍".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, setup_simulation)
        .add_systems(Update, tick_simulation)
        .add_systems(Update, camera_pan_system)
        .add_systems(Update, update_legend_counts)
        .add_systems(Update, update_fog_of_war)
        .add_systems(Update, update_tick_counter)
        .add_systems(Update, update_speed_indicator)
        .add_systems(Update, handle_speed_keyboard)
        .add_systems(Update, handle_speed_buttons)
        .add_systems(Update, handle_play_pause_button)
        .add_systems(Update, update_window_title)
        .run();
}

fn setup_camera(mut commands: Commands) {
    // Calculate the center of the map
    let map_width = MAP_WIDTH as f32 * TILE_SIZE;
    let map_height = MAP_HEIGHT as f32 * TILE_SIZE;
    let center_x = map_width / 2.0;
    let center_y = -map_height / 2.0;

    commands.spawn((
        Camera2d::default(),
        Transform::from_xyz(center_x, center_y, 0.0),
    ));
}
