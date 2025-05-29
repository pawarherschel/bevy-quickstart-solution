use bevy::prelude::*;

mod game;
mod splash;
mod start_menu;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("Bevy Quickstart"),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .enable_state_scoped_entities::<GameState>()
        .add_plugins((
            splash::SplashPlugin,
            start_menu::StartMenuPlugin,
            game::GamePlugin,
        ))
        .run()
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, PartialEq, Eq, Hash, States, Default)]
enum GameState {
    #[default]
    Splash,
    StartMenu,
    Game,
}

#[derive(Resource)]
struct GameAssets {
    player_ship: Handle<Image>,
    player_jet_fire: Handle<Image>,
    asteroid: Handle<Image>,
}
