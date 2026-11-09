use bevy::prelude::*;

mod spawn;

use spawn::render_arena;

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, render_arena);
    }
}
