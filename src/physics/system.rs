const BOUNDS: f32 = 512.;

use bevy::{core::FixedTimestep, prelude::*};

use rand::Rng;

#[derive(Component, Default)]
pub struct Physics2D {
    pub velocity: Vec2,
    pub acceleration: Vec2,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct FixedUpdateStage;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(physics_2d).add_stage_after(
            CoreStage::Update,
            FixedUpdateStage,
            SystemStage::parallel()
                .with_run_criteria(FixedTimestep::step(3.0))
                .with_system(random_acceleration),
        );
    }
}

pub fn physics_2d(time: Res<Time>, mut sprite_position: Query<(&mut Physics2D, &mut Transform)>) {
    for (mut data, mut transform) in sprite_position.iter_mut() {
        // bevy has sufficiently confused borrowck to not know that different struct fields can
        // be accessed independently, so we have to do a sneaky copy of the fields here.
        // TODO: make this a teensy bit faster with unsafe code
        let a = data.acceleration;

        transform.translation.x += data.velocity.x * time.delta_seconds();
        transform.translation.y += data.velocity.y * time.delta_seconds();

        data.velocity += a * time.delta_seconds();

        if (transform.translation.x <= -1. * BOUNDS) || (transform.translation.x >= BOUNDS) {
            data.velocity.x *= -1.0;
        }
        if (transform.translation.y <= -1. * BOUNDS) || (transform.translation.y >= BOUNDS) {
            data.velocity.y *= -1.0;
        }
    }
}

pub fn random_acceleration(mut sprite_physics: Query<&mut Physics2D>) {
    for mut data in sprite_physics.iter_mut() {
        let (x, y): (f32, f32) = rand::thread_rng().gen();
        let a: Vec2 = Vec2::new(x * 2. - 1., y * 2. - 1.);
        data.velocity = a * 100.;
        // data.acceleration = a.scale(50.);
        // data.velocity = data.velocity.scale(0.5);
    }
}
