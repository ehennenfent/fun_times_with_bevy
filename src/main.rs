mod physics;

use bevy::prelude::*;
use physics::{PhysicsPlugin, Physics2D};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            title: "GridWorld3".to_string(),
            width: 1024.0,
            height: 1024.0,
            ..default()
        })
        .add_startup_system(setup)
        .add_plugin(PhysicsPlugin)
        .run();
}


fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(25.0, 25.0)),
            ..default()
        },
        ..default()
    }).insert(Physics2D {
        velocity: Default::default(),
        acceleration: Default::default(),
    });

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.75, 0.25),
            custom_size: Some(Vec2::new(25.0, 25.0)),
            ..default()
        },
        ..default()
    }).insert(Physics2D {
        velocity: Default::default(),
        acceleration: Default::default(),
    });

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.75, 0.25, 0.25),
            custom_size: Some(Vec2::new(25.0, 25.0)),
            ..default()
        },
        ..default()
    }).insert(Physics2D {
        velocity: Default::default(),
        acceleration: Default::default(),
    });
}
