use crate::ai::{Decide, Locals, NextAction};
use crate::{Action, Physics2D};
use bevy::prelude::*;

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(decide_next_action)
            .add_system(perform_next_action.after(decide_next_action));
    }
}

fn decide_next_action(time: Res<Time>, mut q: Query<(&Decide, &mut NextAction)>) {
    for (decider, mut next_action) in q.iter_mut() {
        if let Some(action) = (decider.choose_action)(&time) {
            next_action.action = action;
        }
    }
}

fn perform_next_action(time: Res<Time>, mut q: Query<(&mut Physics2D, &NextAction)>) {
    for (mut physics, action) in q.iter_mut() {
            match &action.action {
                Action::MoveAbsolute(target_pos) => {
                    let along_vector = *target_pos - physics.position;
                    move_along_vec(&time, &mut physics, &along_vector);
                }
                Action::MoveRelative(delta) => move_along_vec(&time, &mut physics, delta),
                Action::Wait => {}
                _ => todo!(),
            }
    }
}

fn move_along_vec(time: &Res<Time>, physics: &mut Physics2D, delta: &Vec2) {
    physics.position += (*delta) * time.delta_seconds()
}
