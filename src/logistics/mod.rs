pub mod system;
pub use system::*;

use bevy::prelude::*;
pub type HP = u32;
pub type EP = f64;

#[derive(Component)]
pub struct Team {
    pub name: String,
}

#[derive(Component, Default)]
pub struct Health {
    pub hp: HP,
    max: HP,
}

#[derive(Component, Default)]
pub struct Energy {
    pub ep: EP,
    max: EP,
}

pub struct DamageEvent {
    target: Entity,
    amount: HP,
}

pub struct HealEvent {
    target: Entity,
    amount: HP,
}

impl Health {
    pub fn full(max: HP) -> Self {
        Health { hp: max, max }
    }

    pub fn empty(max: HP) -> Self {
        Health { hp: 0, max }
    }
}

impl Energy {
    pub fn full(max: EP) -> Self {
        Energy { ep: max, max }
    }

    pub fn empty(max: EP) -> Self {
        Energy { ep: 0., max }
    }
}
