use bevy::prelude::*;

use crate::GameState;

pub struct StartMenuPlugin;

impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::StartMenu), display_start_menu)
            .add_systems(
                Update,
                change_to_gameplay.run_if(in_state(GameState::StartMenu)),
            );
    }
}

fn display_start_menu(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Start,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![
            (
                Text::new("Start Menu"),
                TextFont {
                    font_size: 150.0,
                    ..default()
                }
            ),
            (
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::End,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                children!((
                    Text::new("Press any key to continue..."),
                    TextFont {
                        font_size: 50.0,
                        ..default()
                    }
                ))
            ),
        ],
        StateScoped(GameState::StartMenu),
    ));
}

fn change_to_gameplay(mut next: ResMut<NextState<GameState>>, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.get_just_pressed().next().is_some() {
        next.set(GameState::Gameplay);
    }
}
