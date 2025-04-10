use crate::map::{Map, Cell};
use crate::robot::{Robot, RobotRole, Direction};
use crate::station::Station;
use bevy::prelude::*;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel, MouseMotion};
use bevy::input::ButtonInput;
use bevy::ui::{BackgroundColor, PositionType, Val, UiRect, FlexDirection, AlignItems, JustifyContent, Display};
use std::collections::HashSet;
use crate::config::FOG_OF_WAR;
use std::collections::HashMap;

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
    
    #[derive(Resource)]
    pub struct TickCounter {
        pub count: usize,
    }
    
    impl TickCounter {
        pub fn new() -> Self {
            Self { count: 0 }
        }
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
    pub struct TickCounterSprite;
    
    #[derive(Component)]
    pub struct RobotSprite(pub usize);
    
    #[derive(Component)]
    pub struct DirectionIndicator(pub usize);
    
    #[derive(Component)]
    pub struct StationSprite;
    
    #[derive(Component)]
    struct ResourceLegend;

    // Components for tracking counts in the legend
    #[derive(Component)]
    pub struct ResourceCounter(pub Cell);

    #[derive(Component)]
    pub struct RobotCounter(pub RobotRole);
    
    pub const TILE_SIZE: f32 = 32.0;
    
    // Component for the tick counter display
    #[derive(Component)]
    pub struct TickCounterDisplay;
    
    pub fn setup_simulation(
        mut commands: Commands,
        simulation: Res<SimulationData>,
    ) {
        // Create visible cells set for fog of war
        let mut visible_cells = HashSet::new();
        
        if FOG_OF_WAR {
            // Add station's initial vision
            visible_cells.insert((simulation.station_x, simulation.station_y));

            // Add robots' initial positions
            for robot in &simulation.robots {
                visible_cells.insert((robot.x, robot.y));
            }

            // Add discovered cells from the station
            for (&(x, y), _) in &simulation.station.discovered {
                visible_cells.insert((x, y));
            }
        } else {
            // If FOG_OF_WAR is false, make all cells visible
            for y in 0..simulation.map.height {
                for x in 0..simulation.map.width {
                    visible_cells.insert((x, y));
                }
            }
        }
        
        // Add tick counter in the top right
        commands.spawn((
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(10.0),
                top: Val::Px(10.0),
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.7)),
            TickCounterDisplay,
        )).with_children(|parent| {
            // Tick title
            parent.spawn((
                Node {
                    margin: UiRect::bottom(Val::Px(5.0)),
                    padding: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
            )).with_children(|parent| {
                parent.spawn((
                    Text::new("Tick Counter"),
                ));
            });
            
            // Tick display
            parent.spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    margin: UiRect::bottom(Val::Px(5.0)),
                    padding: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
            )).with_children(|parent| {
                // Icon
                parent.spawn((
                    Node {
                        width: Val::Px(20.0),
                        height: Val::Px(20.0),
                        margin: UiRect::right(Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.6, 1.0)), // Blue
                ));
                
                // Count that will be updated
                parent.spawn((
                    Text::new("0"),
                    TickCounterSprite,
                ));
            });
        });
        
        // Spawn map tiles
        for y in 0..simulation.map.height {
            for x in 0..simulation.map.width {
                let position = Vec3::new(
                    x as f32 * TILE_SIZE,
                    -(y as f32 * TILE_SIZE),
                    0.0,
                );
                
                let is_visible = visible_cells.contains(&(x, y));
                let discovered = is_visible;
                
                // Only render with actual cell colors if the cell is visible
                let color = if is_visible {
                    match simulation.map.grid[y][x] {
                        Cell::Empty => Color::srgb(0.8, 0.8, 0.8),    // Light gray
                        Cell::Obstacle => Color::srgb(0.3, 0.3, 0.3), // Dark gray
                        Cell::Energy => Color::srgb(1.0, 0.8, 0.0),   // Gold
                        Cell::Mineral => Color::srgb(0.6, 0.3, 0.8),  // Purple
                        Cell::Science => Color::srgb(0.0, 0.8, 1.0),  // Cyan
                    }
                } else {
                    Color::srgb(0.0, 0.0, 0.0) // Black for undiscovered cells
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
                    TilePosition { x, y, discovered },
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
                // Text label and count
                parent.spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                )).with_children(|parent| {
                    parent.spawn((
                        Text::new("Energy: "),
                    ));
                    // Count component that will be updated
                    parent.spawn((
                        Text::new("0"),
                        ResourceCounter(Cell::Energy),
                    ));
                });
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
                // Text label and count
                parent.spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                )).with_children(|parent| {
                    parent.spawn((
                        Text::new("Mineral: "),
                    ));
                    // Count component that will be updated
                    parent.spawn((
                        Text::new("0"),
                        ResourceCounter(Cell::Mineral),
                    ));
                });
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
                // Text label and count
                parent.spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                )).with_children(|parent| {
                    parent.spawn((
                        Text::new("Science: "),
                    ));
                    // Count component that will be updated
                    parent.spawn((
                        Text::new("0"),
                        ResourceCounter(Cell::Science),
                    ));
                });
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
                // Text label and count
                parent.spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                )).with_children(|parent| {
                    parent.spawn((
                        Text::new("Explorer Robot: "),
                    ));
                    // Count component that will be updated
                    parent.spawn((
                        Text::new("0"),
                        RobotCounter(RobotRole::Explorer),
                    ));
                });
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
                // Text label and count
                parent.spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                )).with_children(|parent| {
                    parent.spawn((
                        Text::new("Collector Robot: "),
                    ));
                    // Count component that will be updated
                    parent.spawn((
                        Text::new("0"),
                        RobotCounter(RobotRole::Collector),
                    ));
                });
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
                // Text label and count
                parent.spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                )).with_children(|parent| {
                    parent.spawn((
                        Text::new("Scientist Robot: "),
                    ));
                    // Count component that will be updated
                    parent.spawn((
                        Text::new("0"),
                        RobotCounter(RobotRole::Scientist),
                    ));
                });
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
        // Create a map to track how many robots are at each position
        let mut robot_position_count: HashMap<(usize, usize), usize> = HashMap::new();
        
        for (i, robot) in simulation.robots.iter().enumerate() {
            // Count robots at each position
            let position_count = robot_position_count.entry((robot.x, robot.y)).or_insert(0);
            *position_count += 1;
            
            // Calculate offset based on how many robots are already at this position
            let offset_angle = (*position_count as f32 - 1.0) * std::f32::consts::PI / 4.0;
            let offset_distance = if *position_count > 1 { TILE_SIZE * 0.3 } else { 0.0 };
            let offset_x = offset_distance * offset_angle.cos();
            let offset_y = offset_distance * offset_angle.sin();
            
            let robot_pos = Vec3::new(
                robot.x as f32 * TILE_SIZE + offset_x,
                -(robot.y as f32 * TILE_SIZE) + offset_y,
                2.0,
            );
            
            // Spawn the robot sprite
            let robot_entity = commands.spawn((
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
            )).id();
            
            // Calculate direction indicator position
            let indicator_offset = match robot.direction {
                Direction::North => Vec3::new(0.0, TILE_SIZE * 0.3, 0.1),
                Direction::South => Vec3::new(0.0, -TILE_SIZE * 0.3, 0.1),
                Direction::East => Vec3::new(TILE_SIZE * 0.3, 0.0, 0.1),
                Direction::West => Vec3::new(-TILE_SIZE * 0.3, 0.0, 0.1),
            };
            
            // Spawn direction indicator as a child of the robot
            commands.entity(robot_entity).with_children(|parent| {
                parent.spawn((
                    Sprite {
                        color: Color::srgb(1.0, 1.0, 1.0), // White
                        custom_size: Some(Vec2::splat(TILE_SIZE * 0.2)),
                        ..default()
                    },
                    Transform::from_translation(indicator_offset),
                    Visibility::Visible,
                    DirectionIndicator(i),
                ));
            });
        }
    }
    
    pub fn tick_simulation(
        time: Res<Time>,
        mut timer: ResMut<SimulationTickTimer>,
        mut sim: ResMut<SimulationData>,
        mut tick_counter: ResMut<TickCounter>,
        mut param_set: ParamSet<(
            Query<(&mut Transform, &RobotSprite)>,
            Query<(&mut Transform, &DirectionIndicator)>
        )>,
    ) {
        if timer.timer.tick(time.delta()).just_finished() {
            // Increment tick counter
            tick_counter.count += 1;
            
            // Show tick count in console
            println!("⏱️ Tick! #{}", tick_counter.count);
            
            // Create local copies of the values we need
            let station_x = sim.station_x;
            let station_y = sim.station_y;
            
            // Clone map and station outside the loop to avoid borrowing issues
            let mut map_clone = sim.map.clone();
            let mut station_clone = sim.station.clone();
            
            // Update explorer positions in the station
            {
                // Collect references to all robots
                let robot_refs: Vec<&Robot> = sim.robots.iter().collect();
                station_clone.update_explorer_positions(&robot_refs);
            }
            
            // Update robots one at a time
            for robot in &mut sim.robots {
                // Update the robot with the cloned data
                robot.act(&mut map_clone, station_x, station_y, &mut station_clone);
            }
            
            // Update the main simulation with changes
            sim.map = map_clone;
            sim.station = station_clone;
            
            // Track how many robots are at each position
            let mut robot_position_count: HashMap<(usize, usize), usize> = HashMap::new();
            let mut robot_position_index: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
            
            // First, count robots at each position and track their indices
            for (i, robot) in sim.robots.iter().enumerate() {
                *robot_position_count.entry((robot.x, robot.y)).or_insert(0) += 1;
                robot_position_index.entry((robot.x, robot.y)).or_insert_with(Vec::new).push(i);
            }
            
            // Update robot positions in the UI
            {
                let mut robot_query = param_set.p0();
                for (mut transform, robot_sprite) in robot_query.iter_mut() {
                    let robot = &sim.robots[robot_sprite.0];
                    let pos = (robot.x, robot.y);
                    
                    // Find this robot's index among robots at the same position
                    let position_index = robot_position_index.get(&pos)
                        .and_then(|indices| indices.iter().position(|&idx| idx == robot_sprite.0))
                        .unwrap_or(0);
                    
                    // Calculate offset based on position and total robots at that position
                    let total_at_position = robot_position_count.get(&pos).copied().unwrap_or(1);
                    let offset_angle = (position_index as f32) * std::f32::consts::PI / 4.0;
                    let offset_distance = if total_at_position > 1 { TILE_SIZE * 0.3 } else { 0.0 };
                    let offset_x = offset_distance * offset_angle.cos();
                    let offset_y = offset_distance * offset_angle.sin();
                    
                    let new_pos = Vec3::new(
                        robot.x as f32 * TILE_SIZE + offset_x,
                        -(robot.y as f32 * TILE_SIZE) + offset_y,
                        2.0,
                    );
                    transform.translation = new_pos;
                }
            }
            
            // Update direction indicators
            {
                let mut indicator_query = param_set.p1();
                for (mut transform, indicator) in indicator_query.iter_mut() {
                    let robot = &sim.robots[indicator.0];
                    
                    // Update indicator position based on robot's current direction
                    let indicator_offset = match robot.direction {
                        Direction::North => Vec3::new(0.0, TILE_SIZE * 0.3, 0.1),
                        Direction::South => Vec3::new(0.0, -TILE_SIZE * 0.3, 0.1),
                        Direction::East => Vec3::new(TILE_SIZE * 0.3, 0.0, 0.1),
                        Direction::West => Vec3::new(-TILE_SIZE * 0.3, 0.0, 0.1),
                    };
                    
                    transform.translation = indicator_offset;
                }
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

    // System to update resource and robot counts in the legend
    pub fn update_legend_counts(
        simulation: Res<SimulationData>,
        mut param_set: ParamSet<(
            Query<(&mut Text, &ResourceCounter)>,
            Query<(&mut Text, &RobotCounter)>
        )>,
    ) {
        // Count resources in the station
        let mut energy_count = 0;
        let mut mineral_count = 0;
        let mut science_count = 0;

        // Get resources from the station's resources_collected
        for (&resource_type, &count) in simulation.station.resources_collected.iter() {
            match resource_type {
                Cell::Energy => energy_count = count,
                Cell::Mineral => mineral_count = count,
                Cell::Science => science_count = count,
                _ => {}
            }
        }

        // Count robots by role
        let mut explorer_count = 0;
        let mut collector_count = 0;
        let mut scientist_count = 0;

        for robot in &simulation.robots {
            match robot.role {
                RobotRole::Explorer => explorer_count += 1,
                RobotRole::Collector => collector_count += 1,
                RobotRole::Scientist => scientist_count += 1,
            }
        }

        // Update resource count text
        let mut resource_counters = param_set.p0();
        for (mut text, counter) in resource_counters.iter_mut() {
            let new_text = match counter.0 {
                Cell::Energy => energy_count.to_string(),
                Cell::Mineral => mineral_count.to_string(),
                Cell::Science => science_count.to_string(),
                _ => "0".to_string(),
            };
            *text = Text::new(new_text);
        }

        // Update robot count text
        let mut robot_counters = param_set.p1();
        for (mut text, counter) in robot_counters.iter_mut() {
            let new_text = match counter.0 {
                RobotRole::Explorer => explorer_count.to_string(),
                RobotRole::Collector => collector_count.to_string(),
                RobotRole::Scientist => scientist_count.to_string(),
            };
            *text = Text::new(new_text);
        }
    }

    // Component to store tile position for fog of war updates
    #[derive(Component)]
    pub struct TilePosition {
        pub x: usize,
        pub y: usize,
        pub discovered: bool,
    }

    // System to update tile visibility based on fog of war
    pub fn update_fog_of_war(
        simulation: Res<SimulationData>,
        mut tiles_query: Query<(&mut Sprite, &mut TilePosition)>,
    ) {
        // Skip updating fog of war if it's disabled
        if !FOG_OF_WAR {
            // Even if fog of war is disabled, we still need to update resource visibility
            for (mut sprite, pos) in tiles_query.iter_mut() {
                // Check if the cell in the map still has a resource
                // If not, we should update it to show as empty
                if pos.discovered {
                    // Determine the actual current state of the cell
                    // It might have changed if resources were collected
                    sprite.color = match simulation.map.grid[pos.y][pos.x] {
                        Cell::Empty => Color::srgb(0.8, 0.8, 0.8),    // Light gray
                        Cell::Obstacle => Color::srgb(0.3, 0.3, 0.3), // Dark gray
                        Cell::Energy => Color::srgb(1.0, 0.8, 0.0),   // Gold
                        Cell::Mineral => Color::srgb(0.6, 0.3, 0.8),  // Purple
                        Cell::Science => Color::srgb(0.0, 0.8, 1.0),  // Cyan
                    };
                }
            }
            return;
        }
        
        // Create visible cells set
        let mut visible_cells = HashSet::new();
        
        // Add station's initial vision
        visible_cells.insert((simulation.station_x, simulation.station_y));

        // Add robots' initial positions
        for robot in &simulation.robots {
            visible_cells.insert((robot.x, robot.y));
        }

        // Add discovered cells from the station
        for (&(x, y), _) in &simulation.station.discovered {
            visible_cells.insert((x, y));
        }
        
        // Update tile visibility
        for (mut sprite, mut pos) in tiles_query.iter_mut() {
            let is_visible = visible_cells.contains(&(pos.x, pos.y));
            
            if is_visible {
                // Tile is now visible
                if !pos.discovered {
                    // First time seeing this tile
                    pos.discovered = true;
                }
                
                // Always update the color to reflect current state
                // This ensures resources disappear when collected
                sprite.color = match simulation.map.grid[pos.y][pos.x] {
                    Cell::Empty => Color::srgb(0.8, 0.8, 0.8),    // Light gray
                    Cell::Obstacle => Color::srgb(0.3, 0.3, 0.3), // Dark gray
                    Cell::Energy => Color::srgb(1.0, 0.8, 0.0),   // Gold
                    Cell::Mineral => Color::srgb(0.6, 0.3, 0.8),  // Purple
                    Cell::Science => Color::srgb(0.0, 0.8, 1.0),  // Cyan
                };
            } else if !pos.discovered {
                // Tile has not been discovered yet, keep it black
                sprite.color = Color::srgb(0.0, 0.0, 0.0); // Black
            }
        }
    }

    // System to update window title with tick count
    pub fn update_window_title(
        tick_counter: Res<TickCounter>,
        mut windows: Query<&mut Window>,
    ) {
        if let Ok(mut window) = windows.get_single_mut() {
            window.title = format!("Projet Essaim 🌍 - Tick: {}", tick_counter.count);
        }
    }

    pub fn update_tick_counter(
        tick_counter: Res<TickCounter>,
        mut text_query: Query<&mut Text, With<TickCounterSprite>>,
    ) {
        if let Ok(mut text) = text_query.get_single_mut() {
            // Update the text with the current tick count
            *text = Text::new(format!("{}", tick_counter.count));
        }
    }
}
