use std::path::Path;
use rand::Rng;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

mod entity;

const MAX_ENEMIES: i32 = 4;

fn main(){
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, entity::player_movement)
        .add_systems(Update, entity::enemy_movement)
        .add_systems(Update, entity::keep_entities_in_window)
        .run();
}

pub fn setup(
    mut cmd: Commands,
    _window_query: Query<&Window, With<PrimaryWindow>>,
    _asset_server: Res<AssetServer>
){

    let window = _window_query.get_single().unwrap();

    cmd.spawn((
        Transform::from_xyz(window.width()/2.0, window.height()/2.0, 0.0),
        Camera2d{}
    )
    );

    cmd.spawn((
        entity::Speed(300.0),
        entity::SpriteScale(Vec3::splat(2.5)),
        entity::Direction(Vec3::ZERO),
        entity::Player,
        entity::KeepInWindow,
        Transform::from_xyz(window.width()/2.0, window.height()/2.0, 0.0).with_scale(Vec3::splat(5.0)),
        Sprite::from(_asset_server.load(Path::new("img/player.png")))
    ));

    let mut rng = rand::thread_rng();

    let max_x = entity::get_max_x(window.width(), 32.0*2.5);
    let max_y = entity::get_max_y(window.height(), 32.0*2.5);
    for _i in 1..(MAX_ENEMIES-1) {
        cmd.spawn((
            entity::Speed(100.0),
            entity::SpriteScale(Vec3::splat(2.5)),
            entity::Direction(Vec3::ZERO),
            entity::Enemy,
            entity::KeepInWindow,
            Transform::from_xyz(
                rng.gen_range(0..max_x as i32) as f32,
                rng.gen_range(0..max_y as i32) as f32,
                0.0).with_scale(Vec3::splat(5.0)),
            Sprite::from(_asset_server.load(Path::new("img/player.png")))
        ));
    }

}

