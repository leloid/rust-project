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

use bevy::prelude::*;
use bevy_pancam::{PanCam, PanCamPlugin};

#[derive(Resource)]
pub struct SimulationData {
    pub map: Map,
    pub robots: Vec<Robot>,
    pub station_x: usize,
    pub station_y: usize,
}

#[derive(Component)]
struct Tile;

#[derive(Component)]
pub struct RobotSprite(pub usize);

#[derive(Component)]
struct Station;

const TILE_SIZE: f32 = 64.0; 

pub fn setup_simulation(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    sim: Res<SimulationData>,
) {
        // Centrer la caméra en haut à gauche
    // === Centrage & Zoom Auto ===
    let cam_x = (sim.map.width as f32 * TILE_SIZE) / 2.0;
    let cam_y = -(sim.map.height as f32 * TILE_SIZE) / 2.0;

    // Taille totale de la carte
    let map_pixel_width = sim.map.width as f32 * TILE_SIZE;
    let map_pixel_height = sim.map.height as f32 * TILE_SIZE;

    // Taille de la fenêtre Bevy
    let window_width = 1200.0;
    let window_height = 800.0;

    // Calcul du facteur de zoom : le plus petit pour que toute la carte rentre
    let scale_x = window_width / map_pixel_width;
    let scale_y = window_height / map_pixel_height;
    let camera_scale = scale_x.min(scale_y);

    // Spawn caméra avec zoom & centrage
    commands.spawn((
        Camera2dBundle {
            transform: Transform {
                translation: Vec3::new(cam_x, cam_y, 1000.0),
                scale: Vec3::splat(1.0 / camera_scale),
                ..default()
            },
            ..default()
        },
        PanCam::default(), // Attache le composant PanCam à la caméra
    ));
    for y in 0..sim.map.height {
        for x in 0..sim.map.width {
            let color = match sim.map.grid[y][x] {
                Cell::Empty => Color::rgb(0.9, 0.9, 0.9),
                Cell::Obstacle => Color::rgb(0.2, 0.2, 0.2),
                Cell::Energy => Color::rgb(1.0, 1.0, 0.2),
                Cell::Mineral => Color::rgb(0.7, 0.2, 0.7),
                Cell::Science => Color::rgb(0.2, 1.0, 1.0),
            };

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2::splat(TILE_SIZE - 2.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        x as f32 * TILE_SIZE,
                        -(y as f32 * TILE_SIZE),
                        0.0,
                    ),
                    ..default()
                },
                Tile,
            ));
        }
    }

    // Station
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.0, 0.0, 1.0), // BLUE
                custom_size: Some(Vec2::splat(TILE_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(
                sim.station_x as f32 * TILE_SIZE,
                -(sim.station_y as f32 * TILE_SIZE),
                1.0,
            ),
            ..default()
        },
        Station,
    ));

    // Robots
    for (i, robot) in sim.robots.iter().enumerate() {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: match robot.role {
                        RobotRole::Explorer => Color::srgb(0.0, 1.0, 0.0), // GREEN
                        RobotRole::Collector => Color::srgb(1.0, 0.55, 0.0), // ORANGE 
                        RobotRole::Scientist => Color::srgb(0.5, 0.0, 0.5), // PURPLE 
                    },
                    custom_size: Some(Vec2::splat(TILE_SIZE * 0.75)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    robot.x as f32 * TILE_SIZE,
                    -(robot.y as f32 * TILE_SIZE),
                    2.0,
                ),
                ..default()
            },
            RobotSprite(i),
        ));
    }

    // Tick 1s
    commands.insert_resource(Time::<Fixed>::from_duration(Duration::from_secs(1)));
    commands.insert_resource(SimulationTickTimer(Timer::from_seconds(1.0, TimerMode::Repeating)));
}


#[derive(Resource)]
pub struct SimulationTickTimer(Timer);

pub fn tick_simulation(
    time: Res<Time>,
    mut timer: ResMut<SimulationTickTimer>,
    mut sim: ResMut<SimulationData>,
    mut query: Query<(&mut Transform, &RobotSprite)>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        println!("⏱️ Tick!");

        let SimulationData {
            map,
            robots,
            station_x,
            station_y,
        } = &mut *sim;
        
        let station_x = *station_x;
        let station_y = *station_y;
        
        for robot in robots.iter_mut() {
            robot.act(map, station_x, station_y);
        }
        
        for (mut transform, robot_sprite) in &mut query {
            let robot = &sim.robots[robot_sprite.0];
            transform.translation = Vec3::new(
                robot.x as f32 * TILE_SIZE,
                -(robot.y as f32 * TILE_SIZE),
                2.0,
            );
        }
    }


    
}



pub fn camera_zoom_system(
    mut scroll_evr: EventReader<MouseWheel>,
    mut query: Query<&mut Transform, With<Camera2d>>,
) {
    let mut zoom_delta = 0.0;

    for ev in scroll_evr.read() {
        // Sur mac, `Line` est souvent envoyé, donc on adapte
        let step = match ev.unit {
            MouseScrollUnit::Line => ev.y,
            MouseScrollUnit::Pixel => ev.y * 0.05, // Trackpad doux
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
                // Inversion des axes pour un déplacement naturel
                transform.translation.x -= delta.x;
                transform.translation.y += delta.y;
            }
        }
    }
}
