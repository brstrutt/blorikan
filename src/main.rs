use bevy::prelude::*;
mod jetix;
use jetix::JetixLogo;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(JetixLogo::new(asset_server));
}
