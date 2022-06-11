pub mod system;
pub use system::*;

use crate::logistics::HP;
use bevy::prelude::*;

pub enum Action {
    MoveRelative(Vec2),
    MoveAbsolute(Vec2),
    Attack(Entity),
    Heal(HP),
    Charge,
}

#[derive(Component)]
pub struct Decide {
    pub choose_action: fn() -> Option<Action>,
}

#[derive(Component, Default)]
pub struct NextAction {
    pub action: Option<Action>,
}
