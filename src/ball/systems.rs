use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    ball::{
        BALL_SIZE, BALL_SPEED, MAX_BOUNCE_ANGLE, MAX_SPIN, PADDLE_INFLUENCE, SPIN_INFLUENCE,
        SPIN_TRANSFER, components::Ball,
    },
    game::components::GameScore,
    player::components::{Player, Side},
};

pub fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn((
            Ball,
            Mesh2d(meshes.add(Circle::new(BALL_SIZE))),
            MeshMaterial2d(materials.add(Color::srgb(1.0, 0.1, 0.0))),
            Transform::default(),
            RigidBody::Dynamic,
            Collider::circle(BALL_SIZE),
            LinearVelocity(Vec2::new(BALL_SPEED, 0.0)),
            Restitution::new(1.0).with_combine_rule(CoefficientCombine::Max),
            Friction::new(0.0).with_combine_rule(CoefficientCombine::Min),
            CollisionEventsEnabled,
        ))
        .observe(on_paddle_bounce);
}

fn on_paddle_bounce(
    event: On<CollisionStart>,
    mut ball_query: Query<(&mut LinearVelocity, &mut AngularVelocity), With<Ball>>,
    paddle_query: Query<(&LinearVelocity, &Player), Without<Ball>>,
    mut score: ResMut<GameScore>,
) {
    let Ok((mut ball_velocity, mut ball_spin)) = ball_query.get_mut(event.collider1) else {
        return;
    };
    let Ok((paddle_velocity, player)) = paddle_query.get(event.collider2) else {
        return;
    };

    let away = match player.0 {
        Side::Right => {
            score.right += 1;
            -1.0
        }
        Side::Left => {
            score.left += 1;
            1.0
        }
    };

    let base_angle = ball_velocity.y.atan2(ball_velocity.x * away);
    let angle = (base_angle + PADDLE_INFLUENCE * paddle_velocity.y + SPIN_INFLUENCE * ball_spin.0)
        .clamp(-MAX_BOUNCE_ANGLE, MAX_BOUNCE_ANGLE);

    ball_velocity.0 = BALL_SPEED * Vec2::new(away * angle.cos(), angle.sin());
    ball_spin.0 = (SPIN_TRANSFER * paddle_velocity.y).clamp(-MAX_SPIN, MAX_SPIN);
}
