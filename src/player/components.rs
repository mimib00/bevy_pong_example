use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Player(pub Side);

#[derive(Component, Debug)]
pub enum Side {
    Right,
    Left,
}
