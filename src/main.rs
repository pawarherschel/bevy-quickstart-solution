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
        .add_systems(Update, remove_title)
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

    commands.insert_resource(SplashScreenTimer(Timer::from_seconds(2.0, TimerMode::Once)));
}

#[derive(Resource)]
struct SplashScreenTimer(Timer);

fn remove_title(
    #[allow(clippy::needless_pass_by_value)] time: Res<Time>,
    mut timer: ResMut<SplashScreenTimer>,
    mut commands: Commands,
    nodes: Query<Entity, With<Node>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for entity in &nodes {
            commands.entity(entity).despawn();
        }
    }
}
