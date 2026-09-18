use bevy::prelude::*;

use crate::player::PlayerPlugin;

mod player;
mod map;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            file_path: "src/assets".into(),
            ..default()
        }))
        .add_systems(Startup, setup_camera)
        .add_plugins(PlayerPlugin)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
// #[derive(Component)]
// struct Player;

// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins)
//         .add_systems(Startup, setup)
//         .add_systems(Update, move_player)
//         .run();
// }

// fn setup(mut commands: Commands) {
//     commands.spawn(Camera2d);
//     commands.spawn((
//         Text2d::new("@"),
//         TextFont {
//             font_size: FontSize::Px(12.0),
//             font: default(),
//             ..default()
//         },
//         TextColor(Color::WHITE),
//         Transform::from_translation(Vec3::ZERO),
//         Player,
//     ));
// }

// fn move_player(
//     input: Res<ButtonInput<KeyCode>>,
//     time: Res<Time>,
//     mut player_transform: Single<&mut Transform, With<Player>>,
// ) {
//     let mut direction = Vec2::ZERO;
//     if input.pressed(KeyCode::ArrowLeft) | input.pressed(KeyCode::KeyA) {
//         direction.x -= 1.0;
//     }
//     if input.pressed(KeyCode::ArrowRight) | input.pressed(KeyCode::KeyD) {
//         direction.x += 1.0;
//     }
//     if input.pressed(KeyCode::ArrowUp) | input.pressed(KeyCode::KeyW) {
//         direction.y += 1.0;
//     }
//     if input.pressed(KeyCode::ArrowDown) | input.pressed(KeyCode::KeyS) {
//         direction.y -= 1.0;
//     }
//     if direction != Vec2::ZERO {
//         let speed = 300.0;
//         let delta = direction.normalize() * speed * time.delta_secs();
//         player_transform.translation.x += delta.x;
//         player_transform.translation.y += delta.y;
//     }
// }
