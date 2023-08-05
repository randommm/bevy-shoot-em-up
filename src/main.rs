use bevy::prelude::*;
use bevy_shoot_em_up::GamePlugin;

#[cfg(not(feature = "mobile"))]
#[bevy_main]
fn main() {
    App::new().add_plugins((DefaultPlugins, GamePlugin)).run();
}

#[cfg(feature = "mobile")]
#[bevy_main]
fn main() {
    use bevy::window::WindowResolution;

    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(900., 1500.),
                    ..default()
                }),
                ..default()
            }),
            GamePlugin,
        ))
        .run();
}
