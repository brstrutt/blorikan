use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn setup_world(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;

    const SCREEN_WIDTH: f32 = 600.0;
    const SCREEN_HEIGHT: f32 = 400.0;
    const BORDER_THICKNESS: f32 = 100.0;

    commands.spawn((
        Collider::cuboid(SCREEN_WIDTH, BORDER_THICKNESS),
        TransformBundle::from(Transform::from_xyz(0.0, -1.0 * SCREEN_HEIGHT, 0.0)),
        Friction::coefficient(0.0),
        Restitution::coefficient(1.0)
    ));
    
    commands.spawn((
        Collider::cuboid(SCREEN_WIDTH, BORDER_THICKNESS),
        TransformBundle::from(Transform::from_xyz(0.0, SCREEN_HEIGHT, 0.0)),
        Friction::coefficient(0.0),
        Restitution::coefficient(1.0)
    ));
    
    commands.spawn((
        Collider::cuboid(BORDER_THICKNESS, SCREEN_HEIGHT),
        TransformBundle::from(Transform::from_xyz(-1.0 * SCREEN_WIDTH, 0.0, 0.0)),
        Friction::coefficient(0.0),
        Restitution::coefficient(1.0)
    ));

    commands.spawn((
        Collider::cuboid(BORDER_THICKNESS, SCREEN_HEIGHT),
        TransformBundle::from(Transform::from_xyz(SCREEN_WIDTH, 0.0, 0.0)),
        Friction::coefficient(0.0),
        Restitution::coefficient(1.0)
    ));
}
