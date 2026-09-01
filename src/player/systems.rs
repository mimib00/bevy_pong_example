use avian2d::prelude::*;
use bevy::prelude::*;

use crate::player::{
    PADDEL_HEIGHT, PADDEL_WIDTH,
    components::{Player, Side},
};

pub fn spawn_players(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Single<&Window>,
) {
    let width = window.width();

    commands.spawn_batch([
        (
            Player(Side::Right),
            Mesh2d(meshes.add(Rectangle::new(PADDEL_WIDTH, PADDEL_HEIGHT))),
            MeshMaterial2d(materials.add(Color::WHITE)),
            Transform::from_translation(vec3((width / 2.0) - (PADDEL_WIDTH / 2.0), 0.0, 0.0)),
            Collider::rectangle(PADDEL_WIDTH, PADDEL_HEIGHT),
            RigidBody::Dynamic,
            LockedAxes::new().lock_translation_x().lock_rotation(),
            LinearVelocity::ZERO,
            Restitution::ZERO,
            Friction::ZERO,
        ),
        (
            Player(Side::Left),
            Mesh2d(meshes.add(Rectangle::new(PADDEL_WIDTH, PADDEL_HEIGHT))),
            MeshMaterial2d(materials.add(Color::WHITE)),
            Transform::from_translation(vec3(
                ((width / 2.0) - (PADDEL_WIDTH / 2.0)) * -1.0,
                0.0,
                0.0,
            )),
            Collider::rectangle(PADDEL_WIDTH, PADDEL_HEIGHT),
            RigidBody::Dynamic,
            LockedAxes::new().lock_translation_x().lock_rotation(),
            LinearVelocity::ZERO,
            Restitution::ZERO,
            Friction::ZERO,
        ),
    ]);
}
