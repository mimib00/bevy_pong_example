use avian2d::prelude::*;
use bevy::prelude::*;

use crate::game_state::states::GameState;

mod assets;
mod ball;
mod camera;
mod game;
mod game_state;
mod input;
mod menu;
mod player;
mod ui;

fn main() -> AppExit {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)));

    app.add_plugins(DefaultPlugins);

    app.init_state::<GameState>();

    app.add_plugins(PhysicsPlugins::default());
    app.insert_resource(Gravity(Vec2::ZERO));

    app.add_plugins(player::PlayerPlugin);
    app.add_plugins(ball::BallPlugin);
    app.add_plugins(camera::CameraPlugin);
    app.add_plugins(input::InputPlugin);
    app.add_plugins(assets::AssetsLoaderPlugin);
    app.add_plugins(menu::MenuPlugin);
    app.add_plugins(game::GamePlugin);
    app.add_plugins(ui::GameUiPlugin);

    app.run()
}

// const SPEED: f32 = 50.0;

// #[derive(Component)]
// struct Player1;

// fn setup(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
// ) {
//     commands.spawn(Camera2d);

//     commands.spawn((
//         Mesh2d(meshes.add(Rectangle::new(50.0, 100.0))),
//         MeshMaterial2d(materials.add(Color::WHITE)),
//         Transform::default(),
//         Player1,
//     ));
// }

// fn on_movement(
//     mut player1: Single<&mut Transform, With<Player1>>,
//     input: Res<ButtonInput<KeyCode>>,
//     time: Res<Time>,
// ) {
//     if input.pressed(KeyCode::KeyW) {
//         player1.translation.y += SPEED * time.delta_secs();
//     }
//     if input.pressed(KeyCode::KeyS) {
//         player1.translation.y -= SPEED * time.delta_secs();
//     }

//     player1.translation.y = player1.translation.y.clamp(-500.0, 500.0);
// }
