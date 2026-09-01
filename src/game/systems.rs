use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    ball::components::Ball,
    game::{
        WALL_THICKNESS,
        components::{GameOverText, GameWall},
    },
    game_state::states::GameState,
    player::components::Player,
    ui::components::ScoreSide,
};

pub fn spawn_walls(mut commands: Commands, window: Single<&Window>) {
    let height = window.height();
    let width = window.width();

    // --- TOP WALL (Bounces) ---
    commands.spawn((
        GameWall,
        Sprite {
            color: Color::srgba(0.0, 0.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(width * 2.0, WALL_THICKNESS)),

            ..default()
        },
        Transform::from_xyz(0.0, height / 2.0, 0.0),
        RigidBody::Static,
        Collider::rectangle(width * 2.0, WALL_THICKNESS),
        Friction::ZERO,
        Restitution::new(1.0).with_combine_rule(CoefficientCombine::Max),
    ));

    // --- BOTTOM WALL (Bounces) ---
    commands.spawn((
        GameWall,
        Sprite {
            color: Color::srgba(0.0, 0.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(width * 2.0, WALL_THICKNESS)),
            ..default()
        },
        Transform::from_xyz(0.0, -height / 2.0, 0.0),
        RigidBody::Static,
        Friction::ZERO,
        Collider::rectangle(width * 2.0, WALL_THICKNESS),
        Restitution::new(1.0).with_combine_rule(CoefficientCombine::Max),
    ));

    // --- RIGHT WALL (Bounces) ---
    commands
        .spawn((
            GameWall,
            Sprite {
                color: Color::srgba(0.0, 0.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(WALL_THICKNESS, height * 2.0)),

                ..default()
            },
            Transform::from_xyz(width / 2.0, 0.0, 0.0),
            RigidBody::Static,
            Collider::rectangle(WALL_THICKNESS, height * 2.0),
            Restitution::new(1.0).with_combine_rule(CoefficientCombine::Max),
            CollisionEventsEnabled,
        ))
        .observe(on_ball_out_of_bounds);

    // --- LEFT WALL (Bounces) ---
    commands
        .spawn((
            GameWall,
            Sprite {
                color: Color::srgba(0.0, 0.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(WALL_THICKNESS, height * 2.0)),
                ..default()
            },
            Transform::from_xyz(-width / 2.0, 0.0, 0.0),
            RigidBody::Static,
            Collider::rectangle(WALL_THICKNESS, height * 2.0),
            Restitution::new(1.0).with_combine_rule(CoefficientCombine::Max),
            CollisionEventsEnabled,
        ))
        .observe(on_ball_out_of_bounds);
}

fn on_ball_out_of_bounds(
    event: On<CollisionStart>,
    ball_query: Query<&Ball>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let other_entity = event.collider2;
    if ball_query.contains(other_entity) {
        next_state.set(GameState::GameOver);
    }
}

pub fn game_over(
    mut commands: Commands,
    ball_query: Query<Entity, With<Ball>>,
    paddel_query: Query<Entity, With<Player>>,
    wall_query: Query<Entity, With<GameWall>>,
) {
    for ball in ball_query.iter() {
        commands.entity(ball).despawn();
    }
    for paddle in paddel_query.iter() {
        commands.entity(paddle).despawn();
    }
    for wall in wall_query.iter() {
        commands.entity(wall).despawn();
    }

    commands.spawn((
        GameOverText,
        Text::new("GAME OVER"),
        Node {
            margin: auto().horizontal(),
            top: Val::Vh(50.0),
            ..default()
        },
        TextLayout::justify(Justify::Center).with_no_wrap(),
    ));
    commands.spawn((
        GameOverText,
        Text::new("PRESS ENTER TO RETURN TO MENU"),
        Node {
            margin: auto().horizontal(),
            top: Val::Vh(54.0),
            ..default()
        },
        TextLayout::justify(Justify::Center).with_no_wrap(),
    ));
}

pub fn handle_game_over_input(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    text_query: Query<Entity, With<GameOverText>>,
    score_query: Query<Entity, With<ScoreSide>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if input.pressed(KeyCode::Enter) {
        for text in text_query.iter() {
            commands.entity(text).despawn();
        }
        for score in score_query.iter() {
            commands.entity(score).despawn();
        }

        next_state.set(GameState::StartMenu);
    }
}
