use crate::map::{Map, Cell};
use crate::robot::{Robot, RobotRole};
use crate::station::Station;
use bevy::prelude::*;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel, MouseMotion};
use bevy::input::ButtonInput;

pub mod gui {
    use super::*;
    
    #[derive(Resource)]
    pub struct SimulationData {
        pub map: Map,
        pub robots: Vec<Robot>,
        pub station: Station,
        pub station_x: usize,
        pub station_y: usize,
    }
    
    #[derive(Resource)]
    pub struct SimulationTickTimer {
        pub timer: Timer,
    }
    
    impl SimulationTickTimer {
        pub fn new() -> Self {
            Self {
                timer: Timer::new(std::time::Duration::from_millis(500), TimerMode::Repeating),
            }
        }
    }
    
    #[derive(Component)]
    pub struct MapTile;
    
    #[derive(Component)]
    pub struct RobotSprite(pub usize);
    
    #[derive(Component)]
    pub struct StationSprite;
    
    pub const TILE_SIZE: f32 = 32.0;
    
    pub fn setup_simulation(
        mut commands: Commands,
        simulation: Res<SimulationData>,
    ) {
        // Spawn map tiles
        for y in 0..simulation.map.height {
            for x in 0..simulation.map.width {
                let position = Vec3::new(
                    x as f32 * TILE_SIZE,
                    -(y as f32 * TILE_SIZE),
                    0.0,
                );
                
                let color = match simulation.map.grid[y][x] {
                    Cell::Empty => Color::srgb(0.8, 0.8, 0.8),    // Light gray
                    Cell::Obstacle => Color::srgb(0.3, 0.3, 0.3), // Dark gray
                    Cell::Energy => Color::srgb(1.0, 0.8, 0.0),   // Gold
                    Cell::Mineral => Color::srgb(0.6, 0.3, 0.8),  // Purple
                    Cell::Science => Color::srgb(0.0, 0.8, 1.0),  // Cyan
                };
                
                commands.spawn((
                    Sprite {
                        color,
                        custom_size: Some(Vec2::splat(TILE_SIZE - 1.0)),
                        ..default()
                    },
                    Transform::from_translation(position),
                    Visibility::Visible,
                    MapTile,
                ));
            }
        }
        
        // Spawn station
        let station_pos = Vec3::new(
            simulation.station_x as f32 * TILE_SIZE,
            -(simulation.station_y as f32 * TILE_SIZE),
            1.0,
        );
        
        commands.spawn((
            Sprite {
                color: Color::srgb(1.0, 0.0, 0.0), // Red
                custom_size: Some(Vec2::splat(TILE_SIZE * 1.2)),
                ..default()
            },
            Transform::from_translation(station_pos),
            Visibility::Visible,
            StationSprite,
        ));
        
        // Spawn robots
        for (i, robot) in simulation.robots.iter().enumerate() {
            let robot_pos = Vec3::new(
                robot.x as f32 * TILE_SIZE,
                -(robot.y as f32 * TILE_SIZE),
                2.0,
            );
            
            commands.spawn((
                Sprite {
                    color: match robot.role {
                        RobotRole::Explorer => Color::srgb(0.0, 1.0, 0.0),   // Green
                        RobotRole::Collector => Color::srgb(1.0, 0.5, 0.0),  // Orange
                        RobotRole::Scientist => Color::srgb(0.8, 0.0, 0.8),  // Purple
                    },
                    custom_size: Some(Vec2::splat(TILE_SIZE * 0.8)),
                    ..default()
                },
                Transform::from_translation(robot_pos),
                Visibility::Visible,
                RobotSprite(i),
            ));
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
            
            // Create local copies of the values we need
            let station_x = sim.station_x;
            let station_y = sim.station_y;
            
            // Clone map and station outside the loop to avoid borrowing issues
            let mut map_clone = sim.map.clone();
            let mut station_clone = sim.station.clone();
            
            // Update robots one at a time
            for robot in &mut sim.robots {
                // Update the robot with the cloned data
                robot.act(&mut map_clone, station_x, station_y, &mut station_clone);
            }
            
            // Update the main simulation with changes
            sim.map = map_clone;
            sim.station = station_clone;
            
            // Update robot positions in the UI
            for (mut transform, robot_sprite) in &mut query {
                let robot = &sim.robots[robot_sprite.0];
                let new_pos = Vec3::new(
                    robot.x as f32 * TILE_SIZE,
                    -(robot.y as f32 * TILE_SIZE),
                    2.0,
                );
                transform.translation = new_pos;
            }
        }
    }
    
    pub fn camera_zoom_system(
        mut scroll_evr: EventReader<MouseWheel>,
        mut query: Query<&mut OrthographicProjection, With<Camera>>,
    ) {
        let mut zoom_delta = 0.0;
        
        for ev in scroll_evr.read() {
            match ev.unit {
                MouseScrollUnit::Line => {
                    zoom_delta += ev.y * 0.1;
                }
                MouseScrollUnit::Pixel => {
                    zoom_delta += ev.y * 0.001;
                }
            }
        }
        
        if zoom_delta != 0.0 {
            for mut projection in &mut query {
                projection.scale = (projection.scale - zoom_delta).clamp(0.1, 10.0);
            }
        }
    }
    
    pub fn camera_pan_system(
        mouse: Res<ButtonInput<MouseButton>>,
        mut motion_evr: EventReader<MouseMotion>,
        mut query: Query<&mut Transform, With<Camera>>,
    ) {
        if mouse.pressed(MouseButton::Right) {
            let mut pan = Vec2::ZERO;
            
            for ev in motion_evr.read() {
                pan += ev.delta;
            }
            
            if pan != Vec2::ZERO {
                for mut transform in &mut query {
                    transform.translation.x -= pan.x;
                    transform.translation.y += pan.y;
                }
            }
        }
    }
}
