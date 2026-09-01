use bevy::prelude::*;

mod systems;

const MOVE_SPEED: f32 = 500.0;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, systems::read_input);
    }
}
