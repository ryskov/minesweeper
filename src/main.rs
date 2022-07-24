use bevy::log;
use bevy::prelude::*;
use board_plugin::events::*;
use board_plugin::BoardPlugin;

mod resources;
mod components;
mod systems;

use systems::{clear_pause_screen, pause_screen, game_time_system, setup_board};
use resources::{GameTime, AppState};

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

fn main() {
    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        title: "Mine Sweeper".to_string(),
        width: 800.,
        height: 800.,
        ..Default::default()
    })
    .insert_resource(GameTime::new_paused())
    .add_state(AppState::Out);
    app.add_plugins(DefaultPlugins);
    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());
    app.add_plugin(BoardPlugin {
        running_state: AppState::InGame,
    })
    .add_system_set(SystemSet::on_enter(AppState::Paused).with_system(pause_screen))
    .add_system_set(SystemSet::on_exit(AppState::Paused).with_system(clear_pause_screen))
    .add_system_set(SystemSet::on_update(AppState::InGame).with_system(game_time_system))
    .add_system(state_handler)
    .add_system(game_state_handler)
    .add_startup_system(camera_setup)
    .add_startup_system(setup_board);

    app.run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn game_state_handler(
    mut state: ResMut<State<AppState>>,
    mut bomb_explosion_evr: EventReader<BombExplosionEvent>,
) {
    if bomb_explosion_evr.iter().count() > 0 {
        if state.current() == &AppState::InGame {
            state.set(AppState::GameOver).unwrap();
        }
    }
}

fn state_handler(mut state: ResMut<State<AppState>>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::C) {
        log::debug!("clearing detected");
        if state.current() == &AppState::InGame {
            log::info!("clearing game");
            state.set(AppState::Out).unwrap();
        }
    }
    if keys.just_pressed(KeyCode::G) {
        log::debug!("loading detected");
        match state.current() {
            AppState::InGame => {
                state.restart().unwrap();
            }
            AppState::Out => {
                state.set(AppState::InGame).unwrap();
            }
            AppState::GameOver => {
                state.set(AppState::InGame).unwrap();
            }
            _ => (),
        };
    }
    if keys.just_pressed(KeyCode::Escape) {
        match state.current() {
            AppState::InGame => {
                state.push(AppState::Paused).unwrap();
            }
            AppState::Paused => {
                state.pop().unwrap();
            }
            _ => (),
        };
    }
}
