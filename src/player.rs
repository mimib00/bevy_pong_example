use bevy::prelude::*;

use crate::game_state::states::GameState;

pub mod components;
mod systems;

pub const PADDEL_WIDTH: f32 = 20.0;
pub const PADDEL_HEIGHT: f32 = 200.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), systems::spawn_players);
    }
}
