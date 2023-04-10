use bevy::{prelude::*, window::WindowResolution};
use bevy_rapier2d::prelude::*;

mod controls;
mod world;
mod jetix;

use controls::user_controls;
use world::setup_world;
use jetix::setup_jetix;

fn main() {
    // setup wasm logging, but only print warnings and above because rapier floods the logs with info
    let logger_config = wasm_logger::Config::new(log::Level::Warn);
    wasm_logger::init(logger_config);
    log::warn!("App is starting");

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1000., 600.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        //.add_plugin(RapierDebugRenderPlugin::default()) // Display the colliders
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_world)
        .add_startup_system(setup_jetix)
        .add_system(user_controls)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
