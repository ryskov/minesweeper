use crate::resources::GameTime;
use bevy::log;
use bevy::prelude::*;
use board_plugin::events::{TileMarkEvent, TileTriggerEvent};

pub fn game_time_system(
    mut game_time: ResMut<GameTime>,
    time: Res<Time>,
    tile_trigger_evr: EventReader<TileTriggerEvent>,
    tile_mark_trigger_evr: EventReader<TileMarkEvent>,
) {
    if game_time.paused() {
        if tile_trigger_evr.len() > 0 || tile_mark_trigger_evr.len() > 0 {
            log::info!("Game started");
            game_time.unpause();
        }
    }

    if !game_time.paused() {
        game_time.tick(time.delta());
    }
}
