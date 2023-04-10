use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;
use crate::jetix::JetixLogo;

pub fn user_controls(
    keyboard_input: Res<Input<KeyCode>>,
    mut jetix_logo: Query<(& JetixLogo, &mut Velocity)>
) {
    let (_, mut velocity) = jetix_logo.single_mut();

    if keyboard_input.pressed(KeyCode::Left) { velocity.linvel.x /= 1.02; }
    if keyboard_input.pressed(KeyCode::Right) { velocity.linvel.x *= 1.02; }
    if keyboard_input.pressed(KeyCode::Up) { velocity.linvel.y *= 1.02; }
    if keyboard_input.pressed(KeyCode::Down) { velocity.linvel.y /= 1.02; }
}
