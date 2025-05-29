use crate::{GameAssets, GameState};
use bevy::prelude::*;
use log::{Level, log};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), display_level)
            .add_systems(
                FixedUpdate,
                (control_player, collision)
                    .chain()
                    .run_if(in_state(GameState::Game)),
            );
    }
}

#[derive(Component, Deref, DerefMut, Copy, Clone)]
struct Velocity(Vec3);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Asteroid;

fn display_level(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands.spawn((
        Sprite::from_image(game_assets.player_ship.clone()),
        Player,
        Velocity(Vec3::ZERO),
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
    let (mut player_transform, mut velocity, player_children) = player.single_mut()?;

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
        velocity.0 += *forward;
        velocity.0 = velocity.0.clamp_length(0.0, 5.0);
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

    player_transform.translation += velocity.0;

    Ok(())
}

fn collision(
    asteroids: Query<&Transform, With<Asteroid>>,
    player: Query<&Transform, With<Player>>,
    mut gizmos: Gizmos,
) -> Result {
    let player_radius = 40.0;
    let asteroid_radius = 50.0;

    let player_transform = player.single()?;
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
    for asteroid_transform in &asteroids {
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
            #[cfg(debug_assertions)]
            {
                log!(Level::Info, "Collision");
            }
        }
    }

    Ok(())
}
