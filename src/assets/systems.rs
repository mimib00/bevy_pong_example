use bevy::prelude::*;

use crate::game_state::states::GameState;

pub fn check_assets_ready(mut next_state: ResMut<NextState<GameState>>) {
    let all_loaded = true;

    if all_loaded {
        next_state.set(GameState::StartMenu);
    }
}
