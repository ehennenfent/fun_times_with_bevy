const BOUNDS: f32 = 512.;

use bevy::{core::FixedTimestep, prelude::*};

use rand::Rng;

#[derive(Component, Default)]
pub struct Physics2D {
    pub position: Vec2,
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

pub fn physics_2d(time: Res<Time>, mut sprite_position: Query<&mut Physics2D>) {
    for mut data in sprite_position.iter_mut() {
        // bevy has sufficiently confused borrowck to not know that different struct fields can
        // be accessed independently, so we have to do a sneaky copy of the fields here.
        // TODO: make this a teensy bit faster with unsafe code
        let v = data.velocity;
        let a = data.acceleration;

        data.position += v * time.delta_seconds();
        data.velocity += a * time.delta_seconds();

        if (data.position.x <= -1. * BOUNDS) || (data.position.x >= BOUNDS) {
            data.velocity.x *= -1.0;
        }
        if (data.position.y <= -1. * BOUNDS) || (data.position.y >= BOUNDS) {
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
