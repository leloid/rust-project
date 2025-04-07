use bevy::prelude::*;
use crate::map::{Map, Cell};
use crate::robot::{Robot, RobotRole};
use bevy::time::common_conditions::on_timer;
use std::time::Duration;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy::input::ButtonInput;
use crate::station::Station as GameStation;
use bevy_pancam::PanCam;

#[derive(Resource)]
pub struct SimulationData {
    pub map: Map,
    pub robots: Vec<Robot>,
    pub station: GameStation,
    pub station_x: usize,
    pub station_y: usize,
}

#[derive(Component)]
struct Tile;

#[derive(Component)]
pub struct RobotSprite(pub usize);

#[derive(Component)]
struct StationSprite;

const TILE_SIZE: f32 = 32.0; // Reduced tile size for better visibility

pub fn setup_simulation(
    mut commands: Commands,
    _asset_server: Res<AssetServer>,
    sim: Res<SimulationData>,
) {
    println!("Setting up simulation with map size: {}x{}", sim.map.width, sim.map.height);

    // Calculate the world space dimensions
    let world_width = sim.map.width as f32 * TILE_SIZE;
    let world_height = sim.map.height as f32 * TILE_SIZE;
    
    // Center position calculation
    let center_x = world_width / 2.0;
    let center_y = -world_height / 2.0;

    // Camera setup
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(center_x, center_y, 100.0),
            ..default()
        },
        PanCam::default(),
    ));

    // Clear background color
    commands.insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)));

    // Spawn tiles
    for y in 0..sim.map.height {
        for x in 0..sim.map.width {
            let color = match sim.map.grid[y][x] {
                Cell::Empty => Color::rgb(0.8, 0.8, 0.8),    // Light gray
                Cell::Obstacle => Color::rgb(0.3, 0.3, 0.3), // Dark gray
                Cell::Energy => Color::rgb(1.0, 0.8, 0.0),   // Gold
                Cell::Mineral => Color::rgb(0.6, 0.3, 0.8),  // Purple
                Cell::Science => Color::rgb(0.0, 0.8, 1.0),  // Cyan
            };

            let position = Vec3::new(
                x as f32 * TILE_SIZE,
                -(y as f32 * TILE_SIZE),
                0.0,
            );

            println!("Spawning tile at position: {:?}", position);

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2::splat(TILE_SIZE - 1.0)),
                        ..default()
                    },
                    transform: Transform::from_translation(position),
                    visibility: Visibility::Visible,
                    ..default()
                },
                Tile,
            ));
        }
    }

    // Station
    let station_pos = Vec3::new(
        sim.station_x as f32 * TILE_SIZE,
        -(sim.station_y as f32 * TILE_SIZE),
        1.0,
    );

    println!("Spawning station at position: {:?}", station_pos);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 0.0, 1.0), // Blue
                custom_size: Some(Vec2::splat(TILE_SIZE)),
                ..default()
            },
            transform: Transform::from_translation(station_pos),
            visibility: Visibility::Visible,
            ..default()
        },
        StationSprite,
    ));

    // Robots
    for (i, robot) in sim.robots.iter().enumerate() {
        let robot_pos = Vec3::new(
            robot.x as f32 * TILE_SIZE,
            -(robot.y as f32 * TILE_SIZE),
            2.0,
        );

        println!("Spawning robot {} at position: {:?}", i, robot_pos);

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: match robot.role {
                        RobotRole::Explorer => Color::rgb(0.0, 1.0, 0.0),   // Green
                        RobotRole::Collector => Color::rgb(1.0, 0.5, 0.0),  // Orange
                        RobotRole::Scientist => Color::rgb(0.8, 0.0, 0.8),  // Purple
                    },
                    custom_size: Some(Vec2::splat(TILE_SIZE * 0.8)),
                    ..default()
                },
                transform: Transform::from_translation(robot_pos),
                visibility: Visibility::Visible,
                ..default()
            },
            RobotSprite(i),
        ));
    }
}

#[derive(Resource)]
pub struct SimulationTickTimer {
    pub timer: Timer,
}

impl SimulationTickTimer {
    pub fn new() -> Self {
        Self {
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        }
    }
}

pub fn tick_simulation(
    time: Res<Time>,
    mut timer: ResMut<SimulationTickTimer>,
    mut sim: ResMut<SimulationData>,
    mut query: Query<(&mut Transform, &RobotSprite)>,
) {
    if timer.timer.tick(time.delta()).just_finished() {
        println!("⏱️ Tick!");

        let SimulationData {
            map,
            robots,
            station,
            station_x,
            station_y,
        } = &mut *sim;
        
        let station_x = *station_x;
        let station_y = *station_y;
        
        for robot in robots.iter_mut() {
            robot.act(map, station_x, station_y, station);
        }
        
        for (mut transform, robot_sprite) in &mut query {
            let robot = &sim.robots[robot_sprite.0];
            let new_pos = Vec3::new(
                robot.x as f32 * TILE_SIZE,
                -(robot.y as f32 * TILE_SIZE),
                2.0,
            );
            transform.translation = new_pos;
            println!("Moving robot {} to position: {:?}", robot_sprite.0, new_pos);
        }
    }
}

pub fn camera_zoom_system(
    mut scroll_evr: EventReader<MouseWheel>,
    mut query: Query<&mut Transform, With<Camera2d>>,
) {
    let mut zoom_delta = 0.0;

    for ev in scroll_evr.read() {
        let step = match ev.unit {
            MouseScrollUnit::Line => ev.y,
            MouseScrollUnit::Pixel => ev.y * 0.05,
        };
        zoom_delta += step;
    }

    if zoom_delta != 0.0 {
        for mut transform in &mut query {
            let scale = &mut transform.scale;
            let factor = 1.0 - zoom_delta * 0.1;
            let new_scale = (scale.x * factor).clamp(0.1, 5.0);
            *scale = Vec3::splat(new_scale);
        }
    }
}

pub fn camera_pan_system(
    buttons: Res<ButtonInput<MouseButton>>,
    mut motion_evr: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<Camera2d>>,
) {
    if buttons.pressed(MouseButton::Right) {
        let mut delta = Vec2::ZERO;

        for ev in motion_evr.read() {
            delta += ev.delta;
        }

        if delta != Vec2::ZERO {
            for mut transform in &mut query {
                transform.translation.x -= delta.x;
                transform.translation.y += delta.y;
            }
        }
    }
}
