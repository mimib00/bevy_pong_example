use bevy::prelude::*;

use crate::{game::components::GameScore, game_state::states::GameState};

pub mod components;
mod systems;

const WALL_THICKNESS: f32 = 2.0;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameScore::ZERO);
        app.add_systems(OnEnter(GameState::InGame), systems::spawn_walls);
        app.add_systems(OnEnter(GameState::GameOver), systems::game_over);
        app.add_systems(
            Update,
            systems::handle_game_over_input.run_if(in_state(GameState::GameOver)),
        );
    }
}
