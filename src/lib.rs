use bevy::{
    prelude::*,
    sprite::{collide_aabb::collide, MaterialMesh2dBundle},
};
use rand::{rngs::SmallRng, Rng as _, SeedableRng};

mod wall_scoreboard;
use wall_scoreboard::{setup_score_board, setup_walls, update_scoreboard};

pub const PLAYER_SIZE: f32 = 30.0;
pub const ENEMY_SIZE: f32 = 20.0;
pub const BULLET_SIZE: f32 = 5.0;
pub const WALL_THICKNESS: f32 = 10.0;

pub const LEFT_WALL: f32 = -450.;
pub const RIGHT_WALL: f32 = 450.;
pub const BOTTOM_WALL: f32 = -300.;
pub const TOP_WALL: f32 = 300.;

pub const ENEMY_LEFT_BOUNDARY: f32 = LEFT_WALL + WALL_THICKNESS + ENEMY_SIZE / 2.;
pub const ENEMY_RIGHT_BOUNDARY: f32 = RIGHT_WALL - WALL_THICKNESS - ENEMY_SIZE / 2.;
pub const ENEMY_BOTTOM_BOUNDARY: f32 = BOTTOM_WALL + WALL_THICKNESS + ENEMY_SIZE / 2.;
pub const ENEMY_TOP_BOUNDARY: f32 = TOP_WALL - WALL_THICKNESS - ENEMY_SIZE / 2.;

pub const PLAYER_LEFT_BOUNDARY: f32 = LEFT_WALL + WALL_THICKNESS + PLAYER_SIZE / 2.;
pub const PLAYER_RIGHT_BOUNDARY: f32 = RIGHT_WALL - WALL_THICKNESS - PLAYER_SIZE / 2.;
pub const PLAYER_BOTTOM_BOUNDARY: f32 = BOTTOM_WALL + WALL_THICKNESS + PLAYER_SIZE / 2.;
pub const PLAYER_TOP_BOUNDARY: f32 = TOP_WALL - WALL_THICKNESS - PLAYER_SIZE / 2.;

pub const BULLET_LEFT_BOUNDARY: f32 = LEFT_WALL + WALL_THICKNESS + BULLET_SIZE / 2.;
pub const BULLET_RIGHT_BOUNDARY: f32 = RIGHT_WALL - WALL_THICKNESS - BULLET_SIZE / 2.;
pub const BULLET_BOTTOM_BOUNDARY: f32 = BOTTOM_WALL + WALL_THICKNESS + BULLET_SIZE / 2.;
pub const BULLET_TOP_BOUNDARY: f32 = TOP_WALL - WALL_THICKNESS - BULLET_SIZE / 2.;

pub const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);

#[derive(Component, Default, Clone)]
struct Direction {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Bullet;

struct Rng(SmallRng);
impl Default for Rng {
    fn default() -> Self {
        Self(SmallRng::from_entropy())
    }
}

#[derive(Resource)]
struct Sounds {
    collision_bullet_enemy: Handle<AudioSource>,
    game_over: Handle<AudioSource>,
}

#[derive(Default)]
struct CollisionBulletEnemyEvent;

#[derive(Default)]
struct GameOverEvent;

#[derive(Resource, Default)]
pub struct DestroyedEnemyCount(u32);

#[derive(Default)]
struct TimeSince(f32);

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
enum AppState {
    #[default]
    GameStart,
    InGame,
    Paused,
    GameOver,
}

pub fn run_app() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_event::<CollisionBulletEnemyEvent>()
        .add_event::<GameOverEvent>()
        .add_startup_system(setup)
        .add_startup_system(setup_walls)
        .add_startup_system(setup_score_board)
        .add_startup_system(spawn_things)
        .add_system(spawn_player.run_if(in_state(AppState::GameStart)))
        .add_system(
            check_player_collide_enemy
                .before(shoot_bullet)
                .run_if(in_state(AppState::InGame)),
        )
        .add_system(
            shoot_bullet
                .before(move_bullet)
                .run_if(in_state(AppState::InGame)),
        )
        .add_system(
            move_bullet
                .before(move_player)
                .run_if(in_state(AppState::InGame)),
        )
        .add_system(
            move_player
                .before(check_bullet_collide_enemy)
                .run_if(in_state(AppState::InGame)),
        )
        .add_system(
            check_bullet_collide_enemy
                .before(spawn_and_move_enemies)
                .run_if(in_state(AppState::InGame)),
        )
        .add_system(
            play_bullet_collide_enemy_sound
                .after(check_bullet_collide_enemy)
                .run_if(in_state(AppState::InGame)),
        )
        .add_system(play_game_over_sound.after(check_player_collide_enemy))
        .add_system(spawn_and_move_enemies.run_if(in_state(AppState::InGame)))
        .add_system(update_scoreboard.run_if(in_state(AppState::InGame)))
        .add_system(game_restarter.run_if(in_state(AppState::GameOver)))
        .add_system(bevy::window::close_on_esc)
        .init_resource::<DestroyedEnemyCount>()
        .run();
}

fn spawn_things(mut commands: Commands) {
    commands.spawn((Direction::default(),));
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut player_transform = query.single_mut();
    let mut direction_x = 0.0;
    let mut direction_y = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        direction_x -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        direction_x += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        direction_y += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Down) {
        direction_y -= 1.0;
    }

    let new_player_position =
        player_transform.translation.x + direction_x * 600. * time.delta_seconds();
    player_transform.translation.x =
        new_player_position.clamp(PLAYER_LEFT_BOUNDARY, PLAYER_RIGHT_BOUNDARY);

    let new_player_position =
        player_transform.translation.y + direction_y * 600. * time.delta_seconds();
    player_transform.translation.y =
        new_player_position.clamp(PLAYER_BOTTOM_BOUNDARY, PLAYER_TOP_BOUNDARY);
}

fn shoot_bullet(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    keyboard_input: Res<Input<KeyCode>>,
    query: Query<&Transform, With<Player>>,
) {
    let player_transform = query.single();
    let mut direction = Direction::default();

    if keyboard_input.pressed(KeyCode::F) {
        direction.y = 1.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        direction.x = -1.0;
    }
    if keyboard_input.pressed(KeyCode::G) {
        direction.x = 1.0;
    }

    if direction.x == 0.0 && direction.y == 0.0 {
        return;
    }
    commands.spawn((
        direction.clone(),
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform::from_xyz(
                player_transform.translation.x + direction.x * PLAYER_SIZE / 2.,
                player_transform.translation.y + direction.y * PLAYER_SIZE / 2.,
                player_transform.translation.z,
            )
            .with_scale(Vec3::splat(BULLET_SIZE)),
            material: materials.add(ColorMaterial::from(Color::RED)),
            ..default()
        },
        Bullet,
    ));
}

fn move_bullet(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Direction), With<Bullet>>,
    time: Res<Time>,
) {
    for (entity, mut transform, direction) in query.iter_mut() {
        if transform.translation.y > BULLET_TOP_BOUNDARY
            || transform.translation.y < BULLET_BOTTOM_BOUNDARY
            || transform.translation.x > BULLET_RIGHT_BOUNDARY
            || transform.translation.x < BULLET_LEFT_BOUNDARY
        {
            commands.entity(entity).despawn();
        } else {
            transform.translation.x += direction.x * 600. * time.delta_seconds();
            transform.translation.y += direction.y * 600. * time.delta_seconds();
        }
    }
}

fn check_bullet_collide_enemy(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    mut destroyed_enemy_count: ResMut<DestroyedEnemyCount>,
    mut collision_bullet_enemy_event: EventWriter<CollisionBulletEnemyEvent>,
) {
    for (enemy_entity, enemy_transform) in enemy_query.iter() {
        for (bullet_entity, bullet_transform) in bullet_query.iter() {
            let collision_bullet_enemy = collide(
                bullet_transform.translation,
                bullet_transform.scale.truncate(),
                enemy_transform.translation,
                enemy_transform.scale.truncate(),
            );
            if collision_bullet_enemy.is_some() {
                commands.entity(bullet_entity).despawn();
                commands.entity(enemy_entity).despawn();

                destroyed_enemy_count.0 += 1;
                collision_bullet_enemy_event.send_default();
                break;
            }
        }
    }
}

fn play_bullet_collide_enemy_sound(
    mut collision_bullet_enemy_events: EventReader<CollisionBulletEnemyEvent>,
    audio: Res<Audio>,
    sound: Res<Sounds>,
) {
    if !collision_bullet_enemy_events.is_empty() {
        collision_bullet_enemy_events.clear();
        audio.play(sound.collision_bullet_enemy.clone());
    }
}

fn play_game_over_sound(
    mut collision_bullet_enemy_events: EventReader<GameOverEvent>,
    audio: Res<Audio>,
    sound: Res<Sounds>,
) {
    if !collision_bullet_enemy_events.is_empty() {
        collision_bullet_enemy_events.clear();
        audio.play(sound.game_over.clone());
    }
}

fn gen_rand(rng: &mut Rng, min: f32, max: f32) -> f32 {
    rng.0.gen::<f32>() * (max - min) + min
}

// fn randomize_direction(mut direction: &mut Direction, rng: &mut Rng) {
//     direction.x = gen_rand(rng, -1., 1.);
//     direction.y = gen_rand(rng, -1., 1.);
//     normalize_direction(direction);
// }

fn normalize_direction(mut direction: &mut Direction) {
    let norm = (direction.x.powi(2) + direction.y.powi(2)).sqrt() + 1e-10;
    direction.x /= norm;
    direction.y /= norm;
}

#[allow(clippy::too_many_arguments)]
fn spawn_and_move_enemies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query_player: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut query_enemy: Query<(&mut Direction, &mut Transform), With<Enemy>>,
    mut rng: Local<Rng>,
    time: Res<Time>,
    mut time_since: Local<TimeSince>,
    destroyed_enemy_count: Res<DestroyedEnemyCount>,
) {
    let player_position = query_player.single().translation;
    time_since.0 += time.delta_seconds();
    let allow_new_spawn = if time_since.0 > 0.5 {
        time_since.0 = 0.0;
        true
    } else {
        false
    };

    for (mut direction, mut transform) in query_enemy.iter_mut() {
        //if rng.0.gen::<f32>() > 0.95 {
        let enemy_position = transform.translation;
        let new_direction = player_position - enemy_position;
        direction.x = new_direction.x;
        direction.y = new_direction.y;
        normalize_direction(&mut direction);

        if (direction.x > 0. && transform.translation.x > ENEMY_RIGHT_BOUNDARY)
            || (direction.x < 0. && transform.translation.x < ENEMY_LEFT_BOUNDARY)
        {
            direction.x = -direction.x;
        }
        if (direction.y > 0. && transform.translation.y > ENEMY_TOP_BOUNDARY)
            || (direction.y < 0. && transform.translation.y < ENEMY_BOTTOM_BOUNDARY)
        {
            direction.y = -direction.y;
        }
        transform.translation.x += 30.0
            * time.delta_seconds()
            * direction.x
            * (1. + destroyed_enemy_count.0 as f32 * 0.08);
        transform.translation.y += 30.0
            * time.delta_seconds()
            * direction.y
            * (1. + destroyed_enemy_count.0 as f32 * 0.08);
    }

    if query_enemy.iter().len() >= 7 || !allow_new_spawn {
        return;
    }

    //let mut direction = default();
    //randomize_direction(&mut direction, &mut rng);
    let rand = rng.0.gen::<f32>();
    let x;
    let y;
    if rand > 0.66 {
        x = gen_rand(&mut rng, ENEMY_LEFT_BOUNDARY, ENEMY_RIGHT_BOUNDARY);
        y = ENEMY_TOP_BOUNDARY;
    } else if rand > 0.33 {
        x = ENEMY_LEFT_BOUNDARY;
        y = gen_rand(&mut rng, 0., ENEMY_TOP_BOUNDARY);
    } else {
        x = ENEMY_RIGHT_BOUNDARY;
        y = gen_rand(&mut rng, 0., ENEMY_TOP_BOUNDARY);
    }

    commands.spawn((
        Direction::default(),
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform::from_xyz(x, y, 0.).with_scale(Vec3::splat(ENEMY_SIZE)),
            material: materials.add(ColorMaterial::from(Color::MIDNIGHT_BLUE)),
            ..default()
        },
        Enemy,
    ));
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(Sounds {
        // Source: https://pixabay.com/sound-effects/search/public-domain/
        collision_bullet_enemy: asset_server.load("sounds/collision_bullet_enemy.ogg"),
        game_over: asset_server.load("sounds/game_over.ogg"),
    });
}

fn check_player_collide_enemy(
    mut commands: Commands,
    player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    mut app_state: ResMut<NextState<AppState>>,
    mut game_over_event: EventWriter<GameOverEvent>,
) {
    for (player_entity, player_transform) in player_query.iter() {
        for (_, enemy_transform) in enemy_query.iter() {
            let collision = collide(
                player_transform.translation,
                player_transform.scale.truncate(),
                enemy_transform.translation,
                enemy_transform.scale.truncate(),
            );
            if collision.is_some() {
                app_state.set(AppState::GameOver);
                commands.entity(player_entity).despawn();
                game_over_event.send_default();
                break;
            }
        }
    }
}

fn game_restarter(
    mut commands: Commands,
    mut app_state: ResMut<NextState<AppState>>,
    mut query: Query<Entity, With<Enemy>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut destroyed_enemy_count: ResMut<DestroyedEnemyCount>,
) {
    if keyboard_input.pressed(KeyCode::R) {
        for entity in query.iter_mut() {
            commands.entity(entity).despawn();
        }
        app_state.set(AppState::GameStart);
        destroyed_enemy_count.0 = 0;
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    // Swawn Player
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::default().into()).into(),
            material: materials.add(ColorMaterial::from(Color::LIME_GREEN)),
            transform: Transform::from_xyz(0., PLAYER_BOTTOM_BOUNDARY * 0.9, 0.)
                .with_scale(Vec3::splat(PLAYER_SIZE)),
            ..default()
        },
        Player,
    ));
    app_state.set(AppState::InGame);
}
