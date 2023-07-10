// The code in this file is adapted from
// https://github.com/bevyengine/bevy/blob/main/examples/games/breakout.rs
// which is licensed under either of
// License https://github.com/bevyengine/bevy/blob/main/LICENSE-MIT
// License https://github.com/bevyengine/bevy/blob/main/LICENSE-APACHE

use bevy::prelude::*;

use crate::{
    DestroyedEnemyCount, BOTTOM_WALL, LEFT_WALL, RIGHT_WALL, TOP_WALL, WALL_COLOR, WALL_THICKNESS,
};

const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);

const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

#[derive(Component)]
struct Collider;

// This bundle is a collection of the components that define a "wall" in our game
#[derive(Bundle)]
struct WallBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

/// Which side of the arena is this wall located on?
enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.),
            WallLocation::Bottom => Vec2::new(0., BOTTOM_WALL),
            WallLocation::Top => Vec2::new(0., TOP_WALL),
        }
    }

    fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;
        // Make sure we haven't messed up our constants
        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}

impl WallBundle {
    // This "builder method" allows us to reuse logic across our wall entities,
    // making our code easier to read and less prone to bugs when we change the logic
    fn new(location: WallLocation) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    // We need to convert our Vec2 into a Vec3, by giving it a z-coordinate
                    // This is used to determine the order of our sprites
                    translation: location.position().extend(0.0),
                    // The z-scale of 2D objects must always be 1.0,
                    // or their ordering will be affected in surprising ways.
                    // See https://github.com/bevyengine/bevy/issues/4149
                    scale: location.size().extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        }
    }
}

// Add the game's entities to our world
pub fn setup_walls(mut commands: Commands) {
    // Walls
    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
    commands.spawn(WallBundle::new(WallLocation::Top));
}

// fn check_for_collisions(
//     mut commands: Commands,
//     //mut scoreboard: ResMut<Scoreboard>,
//     //mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
//     //collider_query: Query<(Entity, &Transform, Option<&Brick>), With<Collider>>,
//     mut collision_events: EventWriter<CollisionEvent>,
// ) {
//     let (mut ball_velocity, ball_transform) = ball_query.single_mut();
//     let ball_size = ball_transform.scale.truncate();

//     // check collision with walls
//     for (collider_entity, transform, maybe_brick) in &collider_query {
//         let collision = collide(
//             ball_transform.translation,
//             ball_size,
//             transform.translation,
//             transform.scale.truncate(),
//         );
//         if let Some(collision) = collision {
//             // Sends a collision event so that other systems can react to the collision
//             collision_events.send_default();

//             // Bricks should be despawned and increment the scoreboard on collision
//             if maybe_brick.is_some() {
//                 scoreboard.score += 1;
//                 commands.entity(collider_entity).despawn();
//             }

//             // reflect the ball when it collides
//             let mut reflect_x = false;
//             let mut reflect_y = false;

//             // only reflect if the ball's velocity is going in the opposite direction of the
//             // collision
//             match collision {
//                 Collision::Left => reflect_x = ball_velocity.x > 0.0,
//                 Collision::Right => reflect_x = ball_velocity.x < 0.0,
//                 Collision::Top => reflect_y = ball_velocity.y < 0.0,
//                 Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
//                 Collision::Inside => { /* do nothing */ }
//             }

//             // reflect velocity on the x-axis if we hit something on the x-axis
//             if reflect_x {
//                 ball_velocity.x = -ball_velocity.x;
//             }

//             // reflect velocity on the y-axis if we hit something on the y-axis
//             if reflect_y {
//                 ball_velocity.y = -ball_velocity.y;
//             }
//         }
//     }
// }

// Add the game's entities to our world
pub fn setup_score_board(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Scoreboard
    commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    // Source: http://www.publicdomainfiles.com/show_file.php?id=13502542147502
                    font: asset_server.load("fonts/RusticBlackShadow.ttf"),
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: TEXT_COLOR,
                },
            ),
            TextSection::from_style(TextStyle {
                // Source: http://www.publicdomainfiles.com/show_file.php?id=13502542147502
                font: asset_server.load("fonts/RusticBlackShadow.ttf"),
                font_size: SCOREBOARD_FONT_SIZE,
                color: SCORE_COLOR,
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: SCOREBOARD_TEXT_PADDING,
            left: SCOREBOARD_TEXT_PADDING,
            ..default()
        }),
    );
}

pub fn update_scoreboard(
    destroyed_enemy_count: Res<DestroyedEnemyCount>,
    mut query: Query<&mut Text>,
) {
    let mut text = query.single_mut();
    text.sections[1].value = destroyed_enemy_count.0.to_string();
}
