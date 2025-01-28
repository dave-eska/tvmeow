use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Component)]
pub struct Enemy{
    pub speed: f32,
    pub sprite_scale: Vec3,
    pub direction: Vec3,
}

#[derive(Component)]
pub struct Player{
    pub speed: f32,
    pub sprite_scale: Vec3,
    pub direction: Vec3,
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Player), With<Player>>,
    time: Res<Time>
){
    if let Ok(mut player) = player_query.get_single_mut() {
        let input_x = keyboard_input.pressed(KeyCode::KeyD) as i32 - keyboard_input.pressed(KeyCode::KeyA) as i32;
        let input_y = keyboard_input.pressed(KeyCode::KeyW) as i32 - keyboard_input.pressed(KeyCode::KeyS) as i32;

        player.1.direction += Vec3::new(input_x as f32, input_y as f32, 0.0);

        if player.1.direction.length() > 0.0 {
            player.1.direction = player.1.direction.normalize();
        }

        player.0.translation += player.1.direction * player.1.speed * time.delta_secs();

        player.1.direction = Vec3::ZERO;
    }
}

pub fn get_max_x(window_width: f32, entity_width: f32) -> f32 { window_width-entity_width }
pub fn get_max_y(window_height: f32, entity_height: f32) -> f32 { window_height-entity_height }

pub fn keep_entity_in_window(
    mut entity_query: Query<(&mut Transform, &mut Enemy), With<Enemy>>,
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

pub fn enemy_movement(
    mut entity_query: Query<(&mut Transform, &mut Enemy), With<Enemy>>,
    time: Res<Time>
){
    for mut entity in entity_query.iter_mut() {
        let mut translation = entity.0.translation;
    }
}
