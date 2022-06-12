pub mod system;
pub use system::*;

use crate::logistics::HP;
use bevy::prelude::*;

pub type Locals = [u8; 1024*16];

#[derive(Debug)]
pub enum Action {
    MoveRelative(Vec2),
    MoveAbsolute(Vec2),
    Attack(Entity),
    Heal(HP),
    Charge,
    Wait,
}

#[derive(Component)]
pub struct Decide {
    pub choose_action: fn(&Res<Time>, locals: Locals) -> Option<Action>,
    pub local_storage: Locals,
}

#[derive(Component)]
pub struct NextAction {
    pub action: Action,
}

impl Default for NextAction {
    fn default() -> Self {
        NextAction {
            action: Action::Wait,
        }
    }
}
