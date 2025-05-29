use crate::{GameAssets, GameState};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), display_level)
            .add_systems(
                FixedUpdate,
                ((
                    (control_player, slide, wrap_around, collision).chain(),
                    player_explosion.run_if(resource_exists::<PlayerExplosionTimer>),
                )
                    .run_if(in_state(GameState::Game)),),
            );
    }
}

#[derive(Component, Deref, DerefMut, Copy, Clone)]
struct Velocity(Vec3);

fn slide(entities: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in entities {
        transform.translation += velocity.0;
    }
}

#[derive(Component)]
struct WrapAround;

fn wrap_around(
    entities: Query<&mut Transform, With<WrapAround>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) -> Result {
    let window_resolution = &windows.single()?.resolution;

    let window_height = window_resolution.height() / 2.0;
    let window_width = window_resolution.width() / 2.0;

    for mut entity in entities {
        if entity.translation.x < -window_width {
            entity.translation.x = window_width;
        }
        if entity.translation.y < -window_height {
            entity.translation.y = window_height;
        }

        if entity.translation.x > window_width {
            entity.translation.x = -window_width;
        }
        if entity.translation.y > window_height {
            entity.translation.y = -window_height;
        }
    }

    Ok(())
}

#[derive(Component)]
struct Player;

#[derive(Resource)]
struct PlayerExplosionTimer(Timer);

#[derive(Component)]
struct Asteroid;

fn display_level(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands.spawn((
        Sprite::from_image(game_assets.player_ship.clone()),
        Player,
        Velocity(Vec3::ZERO),
        WrapAround,
        children![(
            Visibility::Hidden,
            Sprite::from_image(game_assets.player_jet_fire.clone()),
            Transform::from_xyz(0.0, -40.0, -0.2),
        )],
        StateScoped(GameState::Game),
    ));

    for (x, y) in [-1.0, 1.0]
        .into_iter()
        .flat_map(|x| [1.0, -1.0].map(|y| (x, y)))
    {
        commands.spawn((
            Sprite::from_image(game_assets.asteroid.clone()),
            Transform::from_xyz(300.0 * x, 200.0 * y, 0.0),
            Asteroid,
            WrapAround,
            Velocity(Vec2::from_angle(rand::random()).extend(0.0)),
            StateScoped(GameState::Game),
        ));
    }
}

fn control_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player: Query<(&mut Transform, &mut Velocity, &Children), With<Player>>,
    mut visibility: Query<&mut Visibility>,
    time: Res<Time>,
) -> Result {
    let Ok((mut player_transform, mut velocity, player_children)) = player.single_mut() else {
        return Ok(());
    };

    let fixed_rotation_rate = 0.2;
    let rotation_rate = fixed_rotation_rate / (1.0 / (60.0 * time.delta().as_secs_f32()));

    if keyboard_input.pressed(KeyCode::KeyA) {
        player_transform.rotate_z(rotation_rate);
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        player_transform.rotate_z(-rotation_rate);
    }
    if keyboard_input.pressed(KeyCode::KeyW) {
        let forward = player_transform.local_y();
        velocity.0 += *forward * 0.1;
        velocity.0 = velocity.0.clamp_length(0.0, 10.0);
    }

    for &child in player_children {
        visibility
            .get_mut(child)?
            .set_if_neq(if keyboard_input.pressed(KeyCode::KeyW) {
                Visibility::Visible
            } else {
                Visibility::Hidden
            });
    }

    Ok(())
}

fn collision(
    asteroids: Query<(&Transform, Entity), With<Asteroid>>,
    player: Query<(&Transform, Entity), With<Player>>,
    game_assets: Res<GameAssets>,
    mut commands: Commands,
    mut gizmos: Gizmos,
) -> Result {
    let player_radius = 40.0;
    let asteroid_radius = 50.0;

    let Ok((player_transform, player_entity)) = player.single() else {
        return Ok(());
    };
    #[cfg(debug_assertions)]
    {
        gizmos.circle_2d(
            Isometry2d::from_xy(
                player_transform.translation.x,
                player_transform.translation.y,
            ),
            player_radius,
            Color::linear_rgb(0.0, 0.0, 1.0),
        );
    }
    for (asteroid_transform, asteroid_entity) in &asteroids {
        #[cfg(debug_assertions)]
        {
            gizmos.circle_2d(
                Isometry2d::from_xy(
                    asteroid_transform.translation.x,
                    asteroid_transform.translation.y,
                ),
                asteroid_radius,
                Color::linear_rgb(1.0, 0.0, 0.0),
            );
        }

        let distance = asteroid_transform
            .translation
            .distance_squared(player_transform.translation);
        if distance < (asteroid_radius + player_radius) * (asteroid_radius + player_radius) {
            commands.get_entity(player_entity)?.despawn();
            commands.get_entity(asteroid_entity)?.despawn();
            commands.spawn((
                Sprite::from_image(game_assets.explosion.clone()),
                *player_transform,
                StateScoped(GameState::Game),
            ));
            commands.insert_resource(PlayerExplosionTimer(Timer::from_seconds(
                1.0,
                TimerMode::Once,
            )));
        }
    }

    Ok(())
}

fn player_explosion(
    player_explosion_timer: Option<ResMut<PlayerExplosionTimer>>,
    mut next: ResMut<NextState<GameState>>,
    time: Res<Time>,
) {
    let Some(mut player_explosion_timer) = player_explosion_timer else {
        return;
    };

    if player_explosion_timer.0.tick(time.delta()).just_finished() {
        next.set(GameState::StartMenu);
    }
}
