use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod jetix;
use jetix::JetixLogo;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default()) // Display the colliders
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_world)
        .add_startup_system(setup_jetix)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_world(mut commands: Commands) {
    const SCREEN_WIDTH: f32 = 400.0;
    const SCREEN_HEIGHT: f32 = 300.0;

    commands.spawn((
        Collider::cuboid(SCREEN_WIDTH, 1.0),
        TransformBundle::from(Transform::from_xyz(0.0, -1.0 * SCREEN_HEIGHT, 0.0)),
        Restitution::coefficient(1.0)
    ));
    
    commands.spawn((
        Collider::cuboid(SCREEN_WIDTH, 1.0),
        TransformBundle::from(Transform::from_xyz(0.0, SCREEN_HEIGHT, 0.0)),
        Restitution::coefficient(1.0)
    ));
    
    commands.spawn((
        Collider::cuboid(1.0, SCREEN_HEIGHT),
        TransformBundle::from(Transform::from_xyz(-1.0 * SCREEN_WIDTH, 0.0, 0.0)),
        Restitution::coefficient(1.0)
    ));

    commands.spawn((
        Collider::cuboid(1.0, SCREEN_HEIGHT),
        TransformBundle::from(Transform::from_xyz(SCREEN_WIDTH, 0.0, 0.0)),
        Restitution::coefficient(1.0)
    ));
}

fn setup_jetix(mut commands: Commands, asset_server: Res<AssetServer>) {
    /* Create the bouncing ball. */
    commands
    .spawn((
        RigidBody::Dynamic,
        TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0))
    ))
    .insert(Collider::ball(50.0))
    .insert(Restitution::coefficient(1.0));
}
