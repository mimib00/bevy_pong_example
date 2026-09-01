use bevy::prelude::*;

use crate::game_state::states::GameState;

mod components;
mod systems;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::StartMenu), systems::show_start_menu)
            .add_systems(OnExit(GameState::StartMenu), systems::hide_start_menu);

        app.add_systems(
            Update,
            systems::handle_menu_input.run_if(in_state(GameState::StartMenu)),
        );
    }
}
