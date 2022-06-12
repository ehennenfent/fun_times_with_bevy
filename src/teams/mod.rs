use crate::ai::Decider;
use crate::logistics::{EP, HP};

struct UnitParams {
    starting_energy: EP,
    max_health: HP,
    speed: f32,
}

struct Unit {
    decider: Decider,
    params: UnitParams,
}

struct Team {
    name: &'static str,
    color: (u8, u8, u8),
    units: Vec<Unit>,
}
