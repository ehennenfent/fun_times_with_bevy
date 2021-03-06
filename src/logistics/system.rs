use crate::logistics::{DamageEvent, HealEvent, Health};
use crate::Energy;
use bevy::core::FixedTimestep;
use bevy::prelude::*;
use std::cmp::{max, min};

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct FixedUpdateStage;

pub struct LogisticsPlugin;

impl Plugin for LogisticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(apply_damage)
            .add_system(apply_healing)
            .add_system(death_system)
            .add_stage_after(
                CoreStage::Update,
                FixedUpdateStage,
                SystemStage::parallel()
                    .with_run_criteria(FixedTimestep::step(3.0))
                    // .with_system(random_acceleration)
                    .with_system(damage_if_no_energy),
            );
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

fn death_system(mut commands: Commands, q: Query<(Entity, &Health)>) {
    for (entity, health) in q.iter() {
        if health.hp == 0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn damage_if_no_energy(mut stats: Query<(&mut Health, &Energy)>) {
    for (mut health, energy) in stats.iter_mut() {
        if energy.ep <= 0.0 {
            health.hp -= 1;
        }
    }
}
