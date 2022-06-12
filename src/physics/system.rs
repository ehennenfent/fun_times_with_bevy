const BOUNDS: f32 = 512.;

use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Physics2D {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
}

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(physics_2d);
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
