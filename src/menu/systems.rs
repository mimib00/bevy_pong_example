use bevy::prelude::*;

use crate::{
    game::components::GameScore, game_state::states::GameState, menu::components::StartMenuLayout,
};

pub fn show_start_menu(mut commands: Commands) {
    commands.spawn((
        StartMenuLayout,
        Text::new("PRESS SPACE TO START"),
        Node {
            margin: auto().horizontal(),
            top: Val::Vh(50.0),
            ..default()
        },
        TextLayout::justify(Justify::Center).with_no_wrap(),
    ));
}

pub fn hide_start_menu(
    mut commands: Commands,
    start_menu_query: Query<Entity, With<StartMenuLayout>>,
) {
    for entity in &start_menu_query {
        commands.entity(entity).despawn();
    }
}

pub fn handle_menu_input(
    input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut score: ResMut<GameScore>,
) {
    if input.pressed(KeyCode::Space) {
        score.reset();
        next_state.set(GameState::InGame);
    }
}
