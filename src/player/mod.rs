use bevy::prelude::*;

mod movement;
mod spawn;

pub struct PlayerPlugin;

use movement::{jump_system, look, move_player, set_cursor, toggle_cursor};
use spawn::add_player;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerName(String);

#[derive(Component, Default, Clone)]
pub struct PlayerCamera;

#[derive(Component, Default, Clone)]
pub struct Gun;

#[derive(Component, Debug)]
pub struct Jumps {
    is_jumping: bool,
    remaining: u8,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_player)
            .add_systems(Update, move_player)
            .add_systems(Update, look)
            .add_systems(Update, jump_system)
            .add_systems(Update, toggle_cursor)
            .add_systems(Startup, set_cursor);
    }
}
