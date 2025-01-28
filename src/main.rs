use std::path::Path;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

fn main(){
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, player_movement)
        .run();
}

#[allow(unused)]
#[derive(Component)]
pub struct Player{
    speed: f32,
    sprite_scale: Vec3
}

pub fn setup(
    mut cmd: Commands,
    _window_query: Query<&Window, With<PrimaryWindow>>,
    _asset_server: Res<AssetServer>){

    let window = _window_query.get_single().unwrap();

    cmd.spawn((
        Player{
            speed: 500.0,
            sprite_scale: Vec3::splat(2.5)
        },
        Transform::from_xyz(window.width()/2.0, window.height()/2.0, 0.0).with_scale(Vec3::splat(5.0)),
        Sprite::from(_asset_server.load(Path::new("img/player.png")))
    ));

    cmd.spawn((
        Transform::from_xyz(window.width()/2.0, window.height()/2.0, 0.0),
        Camera2d{}
    )
    );
}

#[allow(unused)]
pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &Player), With<Player>>,
    time: Res<Time>
){
    if let Ok(mut transf_and_player) = player_query.get_single_mut() {
        let input_x = keyboard_input.pressed(KeyCode::KeyD) as i32 - keyboard_input.pressed(KeyCode::KeyA) as i32;
        let input_y = keyboard_input.pressed(KeyCode::KeyW) as i32 - keyboard_input.pressed(KeyCode::KeyS) as i32;

        let mut direction = Vec3::ZERO;
        direction += Vec3::new(input_x as f32, input_y as f32, 0.0);

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transf_and_player.0.translation += direction * transf_and_player.1.speed * time.delta_secs();
    }
}
