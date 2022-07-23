use crate::Board;
use crate::events::{TileTriggerEvent, TileMarkEvent, UncoverAdjacentTilesEvent};

use bevy::input::{mouse::MouseButtonInput, ElementState};
use bevy::log;
use bevy::prelude::*;

pub fn input_handling(
    windows: Res<Windows>,
    board: Res<Board>,
    mut button_evr: EventReader<MouseButtonInput>,
    mut tile_trigger_ewr: EventWriter<TileTriggerEvent>,
    mut tile_mark_ewr: EventWriter<TileMarkEvent>,
    mut uncover_adjacent_tiles_ewr: EventWriter<UncoverAdjacentTilesEvent>,
) {
    let window = windows.get_primary().unwrap();

    for event in button_evr.iter() {
        if let ElementState::Pressed = event.state {
            let position = window.cursor_position();
            if let Some(pos) = position {
                log::trace!("Mouse button pressed: {:?} at {}", event.button, pos);
                let tile_coordinates = board.mouse_position(window, pos);
                if let Some(coordinates) = tile_coordinates {
                    match event.button {
                        MouseButton::Left => {
                            log::info!("Trying to uncover tile on {}", coordinates);
                            tile_trigger_ewr.send(TileTriggerEvent(coordinates));
                        }
                        MouseButton::Right => {
                            log::info!("Trying to mark tile on {}", coordinates);
                            tile_mark_ewr.send(TileMarkEvent(coordinates));
                        }
                        MouseButton::Middle => {
                            log::info!("Trying to uncover adjacent tiles on {}", coordinates);
                            uncover_adjacent_tiles_ewr.send(UncoverAdjacentTilesEvent(coordinates));
                        }
                        _ => (),
                    }
                }
            }
        }
    }
}
