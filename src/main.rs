mod ai;
mod grid;
mod logistics;
mod physics;

const SIZE: f32 = 20.0;

use bevy::prelude::*;

use crate::ai::{Action, AiPlugin, Decide, NextAction};
use crate::logistics::{DamageEvent, Energy, HealEvent, Health};
use grid::GridPlugin;
use logistics::LogisticsPlugin;
use physics::{Physics2D, PhysicsPlugin};

pub fn decide_green() -> Option<Action> {
    Some(Action::MoveAbsolute(Vec2::new(500., 100.)))
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "GridWorld3".to_string(),
            width: 1024.0,
            height: 1024.0,
            ..default()
        })
        .add_event::<DamageEvent>()
        .add_event::<HealEvent>()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_plugin(PhysicsPlugin)
        .add_plugin(GridPlugin)
        .add_plugin(LogisticsPlugin)
        .add_plugin(AiPlugin)
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
            velocity: Vec2::new(20., 40.),
            acceleration: Default::default(),
        })
        .insert(Energy::empty(10.))
        .insert(Health::full(10));

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
        })
        .insert(Decide {
            choose_action: decide_green,
        })
        .insert(NextAction::default());

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
            velocity: Vec2::new(-200., -100.),
            acceleration: Default::default(),
        });
}
