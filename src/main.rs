use bevy::log;
use bevy::prelude::*;
use board_plugin::resources::BoardOptions;
use board_plugin::resources::{BoardAssets, SpriteMaterial};
use board_plugin::BoardPlugin;

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    InGame,
    Paused,
    Out,
}

fn main() {
    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        title: "Mine Sweeper".to_string(),
        width: 800.,
        height: 800.,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins);

    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());

    app.add_state(AppState::Out).add_plugin(BoardPlugin {
        running_state: AppState::InGame,
    });
    app.add_system(state_handler);

    app.add_startup_system(camera_setup)
        .add_startup_system(setup_board);
    app.run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn setup_board(
    mut commands: Commands,
    mut state: ResMut<State<AppState>>,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(BoardOptions {
        map_size: (30, 30),
        bomb_count: 100,
        tile_padding: 1.0,
        safe_start: true,
        ..Default::default()
    });

    commands.insert_resource(BoardAssets {
        label: "Default".to_string(),
        board_material: SpriteMaterial {
            color: Color::WHITE,
            ..Default::default()
        },
        tile_material: SpriteMaterial {
            color: Color::DARK_GRAY,
            ..Default::default()
        },
        covered_tile_material: SpriteMaterial {
            color: Color::GRAY,
            ..Default::default()
        },
        bomb_counter_font: asset_server.load("fonts/pixeled.ttf"),
        bomb_counter_colors: BoardAssets::default_colors(),
        flag_material: SpriteMaterial {
            color: Color::WHITE,
            texture: asset_server.load("sprites/flag.png"),
        },
        bomb_material: SpriteMaterial {
            color: Color::WHITE,
            texture: asset_server.load("sprites/bomb.png"),
        },
    });
    state.set(AppState::InGame).unwrap();
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
