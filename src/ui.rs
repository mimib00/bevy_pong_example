use bevy::prelude::*;

use crate::game_state::states::GameState;

pub mod components;
mod systems;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), systems::setup_game_ui);
        app.add_systems(
            Update,
            systems::update_score_ui.run_if(in_state(GameState::InGame)),
        );
    }
}
