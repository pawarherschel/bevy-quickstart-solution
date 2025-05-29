use bevy::prelude::*;

use crate::{GameAssets, GameState};

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Splash),
            (display_title, load_assets).chain(),
        )
        .add_systems(Update, switch_to_menu.run_if(in_state(GameState::Splash)));
    }
}

#[derive(Resource)]
struct SplashScreenTimer(Timer);

fn display_title(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![
            (
                Text::new("Bevy Quickstart"),
                TextFont {
                    font_size: 130.0,
                    ..default()
                }
            ),
            (
                Text::new("Rust Week 2025"),
                TextFont {
                    font_size: 100.0,
                    ..default()
                }
            )
        ],
        StateScoped(GameState::Splash),
    ));

    commands.insert_resource(SplashScreenTimer(Timer::from_seconds(2.0, TimerMode::Once)));
}

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        player_ship: asset_server.load("playerShip1_green.png"),
        player_jet_fire: asset_server.load("fire07.png"),
        explosion: asset_server.load("explosion00.png"),
        asteroid: asset_server.load("meteorBrown_big1.png"),
    });
}

fn switch_to_menu(
    mut next: ResMut<NextState<GameState>>,
    mut timer: ResMut<SplashScreenTimer>,
    time: Res<Time>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        next.set(GameState::StartMenu);
    }
}
