use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct JetixLogo;

pub fn setup_jetix(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
    .spawn((
        JetixLogo,
        RigidBody::Dynamic,
        Velocity {
            linvel: Vec2 { x: 100.0, y: 50.0 },
            ..default()
        },
        Collider::cuboid(170.0, 105.0),
        LockedAxes::ROTATION_LOCKED,
        Restitution::coefficient(1.0),
        Friction::coefficient(0.0),
        SpriteBundle {
            texture: asset_server.load("Jetix_logo.png"),
            ..default()
        }
    ));
}
