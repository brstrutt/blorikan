use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod jetix;
use jetix::JetixLogo;

fn main() {
    // setup wasm logging, but only print warnings and above because rapier floods the logs with info
    let logger_config = wasm_logger::Config::new(log::Level::Warn);
    wasm_logger::init(logger_config);
    log::warn!("App is starting");

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default()) // Display the colliders
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_world)
        .add_startup_system(setup_jetix)
        .add_system(user_controls)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_world(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;

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
    commands
    .spawn((
        TempName,
        RigidBody::Dynamic,
        Velocity {
            linvel: Vec2 { x: 100.0, y: 50.0 },
            ..default()
        },
        Collider::cuboid(170.0, 105.0),
        Restitution::coefficient(1.0),
        SpriteBundle {
            texture: asset_server.load("Jetix_logo.png"),
            ..default()
        }
    ));
}

#[derive(Component)]
struct TempName;

fn user_controls(
    keyboard_input: Res<Input<KeyCode>>,
    mut jetix_logo: Query<(& TempName, &mut Velocity)>
) {
    let (_, mut velocity) = jetix_logo.single_mut();

    if keyboard_input.pressed(KeyCode::Left) { velocity.linvel.x -= 1.0; }
    if keyboard_input.pressed(KeyCode::Right) { velocity.linvel.x += 1.0; }
    if keyboard_input.pressed(KeyCode::Up) { velocity.linvel.y += 1.0; }
    if keyboard_input.pressed(KeyCode::Down) { velocity.linvel.y -= 1.0; }
}
