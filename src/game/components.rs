use bevy::prelude::*;

#[derive(Resource)]
pub struct GameScore {
    pub left: u16,
    pub right: u16,
}

impl GameScore {
    pub const ZERO: GameScore = GameScore { left: 0, right: 0 };

    pub fn reset(&mut self) {
        self.left = 0;
        self.right = 0;
    }
}

#[derive(Component)]
pub struct GameWall;

#[derive(Component)]
pub struct GameOverText;
