use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct SpriteScale(pub Vec3);

#[derive(Component)]
pub struct Direction(pub Vec3);

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct KeepInWindow;

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player: Single<(&mut Transform, &mut Direction, &Speed), With<Player>>,
    time: Res<Time>
){
    let input_x = keyboard_input.pressed(KeyCode::KeyD) as i32 - keyboard_input.pressed(KeyCode::KeyA) as i32;
    let input_y = keyboard_input.pressed(KeyCode::KeyW) as i32 - keyboard_input.pressed(KeyCode::KeyS) as i32;

    let (mut transform, mut direction, speed) = player.into_inner();

    direction.0 += Vec3::new(input_x as f32, input_y as f32, 0.0);

    if direction.0.length() > 0.0 {
        direction.0 = direction.0.normalize();
    }

    transform.translation += direction.0 * speed.0 * time.delta_secs();

    direction.0 = Vec3::ZERO;
}

pub fn get_max_x(window_width: f32, entity_width: f32) -> f32 { window_width-entity_width }
pub fn get_max_y(window_height: f32, entity_height: f32) -> f32 { window_height-entity_height }

pub fn keep_entities_in_window(
    mut entity_query: Query<(&mut Transform, &SpriteScale), With<KeepInWindow>>,
    _window_query: Query<&Window, With<PrimaryWindow>>,
){
    let window = _window_query.get_single().unwrap();

    for (mut transform, sprite_scale) in entity_query.iter_mut() {
        let mut translation = transform.translation;

        if translation.x > get_max_x(window.width(), 32.0*sprite_scale.0.x) {
            translation.x = get_max_x(window.width(), 32.0*sprite_scale.0.x);
        }else if translation.x < 32.0*sprite_scale.0.x {
            translation.x = 32.0*sprite_scale.0.x;
        }

        if translation.y > get_max_y(window.height(), 32.0*sprite_scale.0.x) {
            translation.y = get_max_y(window.height(), 32.0*sprite_scale.0.x);
        }else if translation.y < 32.0*sprite_scale.0.y {
            translation.y = 32.0*sprite_scale.0.y;
        }

        transform.translation = translation;
    }
}

pub fn enemy_movement(
    mut entity_query: Query<(&mut Transform, &mut Direction, &Speed), With<Enemy>>,
    player: Single<&Transform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>
){
    let plr_transform = player.into_inner();

    for (mut transform, mut direction, speed) in entity_query.iter_mut() {
        let input_x = if plr_transform.translation.x > transform.translation.x {1 as f32} else {-1 as f32};
        let input_y = if plr_transform.translation.y < transform.translation.y {-1 as f32} else {1 as f32};

        //println!("input_x: {0}\ninput_y: {1}", input_x, input_y);

        direction.0 += Vec3::new(input_x, input_y, 0.0);
        transform.translation += direction.0 * speed.0 * time.delta_secs();
        direction.0 = Vec3::ZERO;
    }
}
