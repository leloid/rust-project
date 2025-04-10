use crate::map::{Map, Cell};
use crate::robot::{Robot, RobotRole};
use crate::station::Station;
use bevy::prelude::*;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel, MouseMotion};
use bevy::input::ButtonInput;
use bevy::ui::{BackgroundColor, PositionType, Val, UiRect, FlexDirection, AlignItems};

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
        
        // Add resource legend
        commands.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(10.0),
                top: Val::Px(10.0),
                padding: UiRect::all(Val::Px(10.0)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.1).with_alpha(0.7)),
            ResourceLegend,
        )).with_children(|parent| {
            // Title: Resources
            parent.spawn((
                Node {
                    margin: UiRect::bottom(Val::Px(10.0)),
                    padding: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
            )).with_children(|parent| {
                parent.spawn((
                    Text::new("Resources"),
                ));
            });

            // Energy
            parent.spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    margin: UiRect::bottom(Val::Px(5.0)),
                    padding: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
            )).with_children(|parent| {
                // Colored square
                parent.spawn((
                    Node {
                        width: Val::Px(20.0),
                        height: Val::Px(20.0),
                        margin: UiRect::right(Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(1.0, 0.8, 0.0)), // Gold
                ));
                // Text label
                parent.spawn((
                    Text::new("Energy"),
                ));
            });

            // Mineral
            parent.spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    margin: UiRect::bottom(Val::Px(5.0)),
                    padding: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
            )).with_children(|parent| {
                // Colored square
                parent.spawn((
                    Node {
                        width: Val::Px(20.0),
                        height: Val::Px(20.0),
                        margin: UiRect::right(Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.6, 0.3, 0.8)), // Purple
                ));
                // Text label
                parent.spawn((
                    Text::new("Mineral"),
                ));
            });

            // Science
            parent.spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    margin: UiRect::bottom(Val::Px(5.0)),
                    padding: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
            )).with_children(|parent| {
                // Colored square
                parent.spawn((
                    Node {
                        width: Val::Px(20.0),
                        height: Val::Px(20.0),
                        margin: UiRect::right(Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.0, 0.8, 1.0)), // Cyan
                ));
                // Text label
                parent.spawn((
                    Text::new("Science"),
                ));
            });

            // Title: Entities
            parent.spawn((
                Node {
                    margin: UiRect::new(Val::Px(0.0), Val::Px(0.0), Val::Px(10.0), Val::Px(5.0)),
                    padding: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
            )).with_children(|parent| {
                parent.spawn((
                    Text::new("Entities"),
                ));
            });

            // Station
            parent.spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    margin: UiRect::bottom(Val::Px(5.0)),
                    padding: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
            )).with_children(|parent| {
                // Colored square
                parent.spawn((
                    Node {
                        width: Val::Px(20.0),
                        height: Val::Px(20.0),
                        margin: UiRect::right(Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(1.0, 0.0, 0.0)), // Red
                ));
                // Text label
                parent.spawn((
                    Text::new("Station"),
                ));
            });

            // Robot - Explorer
            parent.spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    margin: UiRect::bottom(Val::Px(5.0)),
                    padding: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
            )).with_children(|parent| {
                // Colored square
                parent.spawn((
                    Node {
                        width: Val::Px(20.0),
                        height: Val::Px(20.0),
                        margin: UiRect::right(Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.0, 1.0, 0.0)), // Green
                ));
                // Text label
                parent.spawn((
                    Text::new("Explorer Robot"),
                ));
            });

            // Robot - Collector
            parent.spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    margin: UiRect::bottom(Val::Px(5.0)),
                    padding: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
            )).with_children(|parent| {
                // Colored square
                parent.spawn((
                    Node {
                        width: Val::Px(20.0),
                        height: Val::Px(20.0),
                        margin: UiRect::right(Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(1.0, 0.5, 0.0)), // Orange
                ));
                // Text label
                parent.spawn((
                    Text::new("Collector Robot"),
                ));
            });

            // Robot - Scientist
            parent.spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    margin: UiRect::bottom(Val::Px(5.0)),
                    padding: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
            )).with_children(|parent| {
                // Colored square
                parent.spawn((
                    Node {
                        width: Val::Px(20.0),
                        height: Val::Px(20.0),
                        margin: UiRect::right(Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.8, 0.0, 0.8)), // Purple
                ));
                // Text label
                parent.spawn((
                    Text::new("Scientist Robot"),
                ));
            });
        });
        
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

    pub fn camera_pan_system(
        buttons: Res<ButtonInput<MouseButton>>,
        mut motion_evr: EventReader<MouseMotion>,
        mut scroll_evr: EventReader<MouseWheel>,
        mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera2d>>,
    ) {
        // Handle pan
        if buttons.pressed(MouseButton::Left) {
            let mut delta = Vec2::ZERO;

            for ev in motion_evr.read() {
                delta += ev.delta;
            }

            if delta != Vec2::ZERO {
                for (mut transform, _) in &mut query {
                    // Scale the movement to make it more responsive
                    let scale = 2.0;
                    transform.translation.x -= delta.x * scale;
                    transform.translation.y += delta.y * scale;
                }
            }
        }

        // Handle zoom
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
            for (_, mut projection) in &mut query {
                projection.scale = (projection.scale - zoom_delta).clamp(0.1, 10.0);
            }
        }
    }

    #[derive(Component)]
    struct ResourceLegend;
}
