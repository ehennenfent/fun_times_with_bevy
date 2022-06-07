mod grid;
mod physics;

const SIZE: f32 = 20.0;

use bevy::prelude::*;
use grid::GridPlugin;
use physics::{Physics2D, PhysicsPlugin};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "GridWorld3".to_string(),
            width: 1024.0,
            height: 1024.0,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_plugin(PhysicsPlugin)
        .add_plugin(GridPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(SIZE, SIZE)),
                ..default()
            },
            ..default()
        })
        .insert(Physics2D {
            position: Default::default(),
            velocity: Default::default(),
            acceleration: Default::default(),
        });

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.75, 0.25),
                custom_size: Some(Vec2::new(SIZE, SIZE)),
                ..default()
            },
            ..default()
        })
        .insert(Physics2D {
            position: Default::default(),
            velocity: Default::default(),
            acceleration: Default::default(),
        });

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.75, 0.25, 0.25),
                custom_size: Some(Vec2::new(SIZE, SIZE)),
                ..default()
            },
            ..default()
        })
        .insert(Physics2D {
            position: Default::default(),
            velocity: Default::default(),
            acceleration: Default::default(),
        });
}
