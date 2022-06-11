use std::cmp::{min, max};
use crate::logistics::{ChargeEvent, DamageEvent, Energy, HealEvent, Health};
use bevy::prelude::*;

pub struct LogisticsPlugin;

impl Plugin for LogisticsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(apply_damage)
            .add_system(apply_healing);
    }
}

fn apply_damage(mut damage_reader: EventReader<DamageEvent>, mut q: Query<&mut Health>) {
    for damage in damage_reader.iter() {
        if let Ok(mut health) = q.get_mut(damage.target) {
            health.hp = max(health.hp - damage.amount, 0);
        }
    }
}

fn apply_healing(mut heal_reader: EventReader<HealEvent>, mut q: Query<&mut Health>) {
    for heal in heal_reader.iter() {
        if let Ok(mut health) = q.get_mut(heal.target) {
            health.hp = min(health.hp + heal.amount, health.max);
        }
    }
}
