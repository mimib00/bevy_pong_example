use avian2d::dynamics::rigid_body::LinearVelocity;
use bevy::prelude::*;

use crate::{
    input::MOVE_SPEED,
    player::components::{Player, Side},
};

pub fn read_input(
    mut players: Query<(&mut LinearVelocity, &Player)>,

    input: Res<ButtonInput<KeyCode>>,
) {
    for (mut linear_velocity, player) in players.iter_mut() {
        match player.0 {
            Side::Right => right_input(&mut linear_velocity, &input),
            Side::Left => left_input(&mut linear_velocity, &input),
        }
    }
}

fn left_input(velocity: &mut LinearVelocity, input: &ButtonInput<KeyCode>) {
    let mut direction = 0.0;

    if input.pressed(KeyCode::KeyW) {
        direction += 1.0;
    }
    if input.pressed(KeyCode::KeyS) {
        direction -= 1.0;
    }

    velocity.y = direction * MOVE_SPEED;
}

fn right_input(velocity: &mut LinearVelocity, input: &ButtonInput<KeyCode>) {
    let mut direction = 0.0;

    if input.pressed(KeyCode::ArrowUp) {
        direction += 1.0;
    }
    if input.pressed(KeyCode::ArrowDown) {
        direction -= 1.0;
    }
    velocity.y = direction * MOVE_SPEED;
}
