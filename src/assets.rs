use bevy::prelude::*;

use crate::{assets::systems::check_assets_ready, game_state::states::GameState};

mod systems;

pub struct AssetsLoaderPlugin;

impl Plugin for AssetsLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            check_assets_ready.run_if(in_state(GameState::LoadingAssets)),
        );
    }
}
