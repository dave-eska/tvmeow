use std::path::Path;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

fn main(){
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, player_movement)
        .add_systems(Update, keep_entity_in_window)
        .run();
}
#[allow(unused)]
#[derive(Component)]
pub struct Entity{
    speed: f32,
    sprite_scale: Vec3,
    direction: Vec3,
    is_player: bool
}

pub fn setup(
    mut cmd: Commands,
    _window_query: Query<&Window, With<PrimaryWindow>>,
    _asset_server: Res<AssetServer>){

    let window = _window_query.get_single().unwrap();

    cmd.spawn((
        Entity{
            speed: 500.0,
            sprite_scale: Vec3::splat(2.5),
            direction: Vec3::ZERO,
            is_player: true
        },
        Transform::from_xyz(window.width()/2.0, window.height()/2.0, 0.0).with_scale(Vec3::splat(5.0)),
        Sprite::from(_asset_server.load(Path::new("img/player.png")))
    ));

    cmd.spawn((
        Entity{
            speed: 500.0,
            sprite_scale: Vec3::splat(2.5),
            direction: Vec3::ZERO,
            is_player: false
        },
        Transform::from_xyz(window.width()/2.0+300.0, window.height()/2.0, 0.0).with_scale(Vec3::splat(5.0)),
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
    mut entity_query: Query<(&mut Transform, &mut Entity), With<Entity>>,
    time: Res<Time>
){
    for mut player in entity_query.iter_mut() {
        if player.1.is_player {
            let input_x = keyboard_input.pressed(KeyCode::KeyD) as i32 - keyboard_input.pressed(KeyCode::KeyA) as i32;
            let input_y = keyboard_input.pressed(KeyCode::KeyW) as i32 - keyboard_input.pressed(KeyCode::KeyS) as i32;

            player.1.direction += Vec3::new(input_x as f32, input_y as f32, 0.0);

            if player.1.direction.length() > 0.0 {
                player.1.direction = player.1.direction.normalize();
            }

            player.0.translation += player.1.direction * player.1.speed * time.delta_secs();
        }
    }
}

pub fn get_max_x(window_width: f32, entity_width: f32) -> f32 { window_width-entity_width }
pub fn get_max_y(window_height: f32, entity_height: f32) -> f32 { window_height-entity_height }

#[allow(unused)]
pub fn keep_entity_in_window(
    mut entity_query: Query<(&mut Transform, &mut Entity), With<Entity>>,
    _window_query: Query<&Window, With<PrimaryWindow>>,
){
    let window = _window_query.get_single().unwrap();

    for mut entity in entity_query.iter_mut() {
        let mut translation = entity.0.translation;

        if translation.x > get_max_x(window.width(), 32.0*entity.1.sprite_scale.x) {
            translation.x = get_max_x(window.width(), 32.0*entity.1.sprite_scale.x);
        }else if translation.x < 32.0*entity.1.sprite_scale.x {
            translation.x = 32.0*entity.1.sprite_scale.x;
        }

        if translation.y > get_max_y(window.height(), 32.0*entity.1.sprite_scale.x) {
            translation.y = get_max_y(window.height(), 32.0*entity.1.sprite_scale.x);
        }else if translation.y < 32.0*entity.1.sprite_scale.y {
            translation.y = 32.0*entity.1.sprite_scale.y;
        }

        entity.0.translation = translation;
    }
}
