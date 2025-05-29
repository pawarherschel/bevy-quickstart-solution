use bevy::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("Bevy Quickstart"),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, display_title)
        .run()
}

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
    ));
}
