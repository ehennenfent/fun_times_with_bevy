pub mod system;
pub use system::*;

use bevy::prelude::*;
pub type HP = u32;

#[derive(Component)]
pub struct Health {
    hp: HP,
    max: HP,
}

#[derive(Component)]
pub struct Energy {
    ep: HP,
    max: HP,
}

pub struct DamageEvent {
    target: Entity,
    amount: HP,
}

pub struct HealEvent {
    target: Entity,
    amount: HP,
}

