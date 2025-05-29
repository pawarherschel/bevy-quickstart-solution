use crate::{GameAssets, GameState};
use avian2d::prelude::{
    AngularDamping, AngularVelocity, Collider, Collisions, LinearVelocity, RigidBody,
};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;
use std::f32::consts::TAU;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), display_level)
            .add_systems(
                FixedUpdate,
                ((
                    (control_player, wrap_around, collision).chain(),
                    player_explosion.run_if(resource_exists::<PlayerExplosionTimer>),
                )
                    .run_if(in_state(GameState::Game)),),
            );
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
        RigidBody::Dynamic,
        Collider::circle(40.0),
        AngularDamping(5.0),
        Player,
        WrapAround,
        children![(
            Visibility::Hidden,
            Sprite::from_image(game_assets.player_jet_fire.clone()),
            Transform::from_xyz(0.0, -40.0, -0.2),
        )],
        StateScoped(GameState::Game),
    ));

    let mut rng = rand::thread_rng();

    for (x, y) in [-1.0, 1.0]
        .into_iter()
        .flat_map(|x| [1.0, -1.0].map(|y| (x, y)))
    {
        commands.spawn((
            Sprite::from_image(game_assets.asteroid.clone()),
            Transform::from_xyz(300.0 * x, 200.0 * y, 0.0),
            RigidBody::Dynamic,
            Collider::circle(50.0),
            LinearVelocity(Vec2::from_angle(rng.gen_range(0.0..TAU)) * rng.gen_range(10.0..100.0)),
            AngularVelocity(
                rng.gen_range(-std::f32::consts::FRAC_PI_2..std::f32::consts::FRAC_PI_2),
            ),
            Asteroid,
            WrapAround,
            StateScoped(GameState::Game),
        ));
    }
}

fn control_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player: Query<
        (
            &mut Transform,
            &mut LinearVelocity,
            &mut AngularVelocity,
            &Children,
        ),
        With<Player>,
    >,
    mut visibility: Query<&mut Visibility>,
) -> Result {
    let Ok((player_transform, mut linear_velocity, mut angular_velocity, player_children)) =
        player.single_mut()
    else {
        return Ok(());
    };

    if keyboard_input.pressed(KeyCode::KeyA) {
        angular_velocity.0 += 0.4;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        angular_velocity.0 -= 0.4;
    }
    if keyboard_input.pressed(KeyCode::KeyW) {
        let forward = player_transform.local_y();
        linear_velocity.0 += forward.xy() * 4.0;
        linear_velocity.0 = linear_velocity.0.clamp_length_max(300.0);
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
    collisions: Collisions,
    player: Query<(&Transform, Entity), With<Player>>,
    game_assets: Res<GameAssets>,
    mut commands: Commands,
) -> Result {
    let Ok((player_transform, player_entity)) = player.single() else {
        return Ok(());
    };

    for collision in collisions.collisions_with(player_entity) {
        if let Some(body1) = collision.body1 {
            commands.get_entity(body1)?.despawn();
        }
        if let Some(body2) = collision.body2 {
            commands.get_entity(body2)?.despawn();
        }

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
