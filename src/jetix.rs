use bevy::prelude::*;

#[derive(Bundle)]
pub struct JetixLogo {
    sprite: SpriteBundle
}

impl JetixLogo {
    pub fn new(asset_server: Res<AssetServer>) -> JetixLogo {
        JetixLogo {
            sprite: SpriteBundle {
                texture: asset_server.load("Jetix_logo.png"),
                ..default()
            }
        }
    }
}
