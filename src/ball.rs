use bevy::prelude::*;

use crate::game_state::states::GameState;

pub mod components;
mod systems;

const BALL_SIZE: f32 = 15.0;
const BALL_SPEED: f32 = 700.0;
const MAX_BOUNCE_ANGLE: f32 = 1.309;
const PADDLE_INFLUENCE: f32 = 0.0012;
const SPIN_INFLUENCE: f32 = 0.0006;
const SPIN_TRANSFER: f32 = 1.0;
const MAX_SPIN: f32 = 1000.0;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), systems::spawn_ball);
    }
}
