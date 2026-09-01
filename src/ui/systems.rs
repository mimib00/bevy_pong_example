use bevy::prelude::*;

use crate::{game::components::GameScore, ui::components::ScoreSide};

pub fn setup_game_ui(mut commands: Commands) {
    commands.spawn_batch([
        (
            Text::new("0"),
            ScoreSide::Right,
            TextFont {
                font_size: FontSize::Px(50.0),
                ..default()
            },
            Node {
                margin: auto().horizontal(),
                top: Val::Vh(10.0),
                left: Val::Vw(5.0),
                ..default()
            },
            TextLayout::justify(Justify::Center).with_no_wrap(),
        ),
        (
            Text::new("0"),
            ScoreSide::Left,
            TextFont {
                font_size: FontSize::Px(50.0),
                ..default()
            },
            Node {
                margin: auto().horizontal(),
                top: Val::Vh(10.0),
                left: Val::Vw(-5.0),

                ..default()
            },
            TextLayout::justify(Justify::Center).with_no_wrap(),
        ),
    ]);
}

pub fn update_score_ui(score: Res<GameScore>, text_query: Query<(&mut Text, &ScoreSide)>) {
    for (mut text, side) in text_query {
        match side {
            ScoreSide::Right => **text = score.right.to_string(),
            ScoreSide::Left => **text = score.left.to_string(),
        }
    }
}
