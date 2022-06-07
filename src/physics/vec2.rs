use std::ops::AddAssign;

use rand::Rng;
use rand::distributions::{Distribution, Standard};

#[derive(Copy, Clone, Debug, Default)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl AddAssign<Self> for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Vec2 {
    pub fn scale(&self, scalar: f32) -> Vec2 {
        Vec2{x: self.x * scalar, y: self.y * scalar}
    }
}

impl Distribution<Vec2> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec2 {
        let (rand_x, rand_y): (f32, f32) = rng.gen();
        Vec2 {
            x: rand_x * 2.0 - 1.0,
            y: rand_y * 2.0 - 1.0,
        }
    }
}