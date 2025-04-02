// src/resources/gui.rs
use bevy::prelude::*;
use crate::map::{Map, Cell};
use crate::robot::{Robot, RobotRole};

const TILE_SIZE: f32 = 24.0;

#[derive(Resource)]
pub struct SimulationData {
    pub map: Map,
    pub robots: Vec<Robot>,
    pub station_x: usize,
    pub station_y: usize,
}

#[derive(Component)]
pub struct Tile;

pub fn setup_simulation(mut commands: Commands, data: Res<SimulationData>) {
    // Caméra
    commands.spawn(Camera2dBundle::default());

    // Affiche la grille
    for y in 0..data.map.height {
        for x in 0..data.map.width {
            let color = match data.map.grid[y][x] {
                Cell::Empty => Color::rgb(0.9, 0.9, 0.9),
                Cell::Obstacle => Color::rgb(0.3, 0.3, 0.3),
                Cell::Energy => Color::YELLOW,
                Cell::Mineral => Color::PURPLE,
                Cell::Science => Color::CYAN,
            };

            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::splat(TILE_SIZE - 2.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(
                    x as f32 * TILE_SIZE,
                    -(y as f32 * TILE_SIZE),
                    0.0,
                )),
                ..default()
            }).insert(Tile);
        }
    }

    // Station
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::BLUE,
            custom_size: Some(Vec2::splat(TILE_SIZE - 2.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(
            data.station_x as f32 * TILE_SIZE,
            -(data.station_y as f32 * TILE_SIZE),
            1.0,
        )),
        ..default()
    });

    // Robots
    for robot in &data.robots {
        let color = match robot.role {
            RobotRole::Explorer => Color::GREEN,
            RobotRole::Collector => Color::ORANGE,
            RobotRole::Scientist => Color::PINK,
        };

        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::splat(TILE_SIZE - 4.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                robot.x as f32 * TILE_SIZE,
                -(robot.y as f32 * TILE_SIZE),
                2.0,
            )),
            ..default()
        });
    }
}
