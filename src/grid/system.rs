use bevy::prelude::*;

use crate::physics::Physics2D;

const GRID_SIZE: f32 = 5.0;

fn nearest_multiple(i: f32, n: f32) -> f32 {
    let n_trunc = n.trunc();
    let nearest_up = n_trunc * (i / n_trunc).ceil();
    let nearest_down = n_trunc * (i / n_trunc).floor();

    if (nearest_up - i).abs() > (nearest_down - i).abs() {
        nearest_down
    } else {
        nearest_up
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct FixedUpdateStage;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(snap_to_grid);
    }
}

pub fn snap_to_grid(mut sprite_position: Query<(&Physics2D, &mut Transform)>) {
    for (data, mut transform) in sprite_position.iter_mut() {
        transform.translation.x = nearest_multiple(data.position.x, GRID_SIZE);
        transform.translation.y = nearest_multiple(data.position.y, GRID_SIZE);
    }
}
