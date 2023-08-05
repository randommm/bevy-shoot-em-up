// The code in this file is adapted from
// https://github.com/bevyengine/bevy/blob/main/examples/games/breakout.rs
// which is licensed under either of
// License https://github.com/bevyengine/bevy/blob/main/LICENSE-MIT
// License https://github.com/bevyengine/bevy/blob/main/LICENSE-APACHE

use bevy::prelude::*;

#[cfg(feature = "mobile")]
use crate::PRE_BOTTOM_WALL;
use crate::{
    DestroyedEnemyCount, LEFT_WALL, RIGHT_WALL, TOP_WALL, TRUE_BOTTOM_WALL, WALL_COLOR,
    WALL_THICKNESS,
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
    #[cfg(feature = "mobile")]
    PreBottom,
    TrueBottom,
    Top,
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        let lr_y_pos = (TOP_WALL + TRUE_BOTTOM_WALL) / 2.;
        let ub_x_pos = (RIGHT_WALL + LEFT_WALL) / 2.;
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL, lr_y_pos),
            WallLocation::Right => Vec2::new(RIGHT_WALL, lr_y_pos),
            #[cfg(feature = "mobile")]
            WallLocation::PreBottom => Vec2::new(ub_x_pos, PRE_BOTTOM_WALL),
            WallLocation::TrueBottom => Vec2::new(ub_x_pos, TRUE_BOTTOM_WALL),
            WallLocation::Top => Vec2::new(ub_x_pos, TOP_WALL),
        }
    }

    fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL - TRUE_BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;
        // Make sure we haven't messed up our constants
        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
            }
            WallLocation::Top | WallLocation::TrueBottom => {
                Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
            }
            #[cfg(feature = "mobile")]
            WallLocation::PreBottom => Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS),
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
    #[cfg(feature = "mobile")]
    commands.spawn(WallBundle::new(WallLocation::PreBottom));
    commands.spawn(WallBundle::new(WallLocation::TrueBottom));
    commands.spawn(WallBundle::new(WallLocation::Top));
}

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
