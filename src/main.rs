use bevy::core::Stopwatch;
use bevy::log;
use bevy::math::vec2;
use bevy::prelude::*;
use board_plugin::events::*;
use board_plugin::resources::BoardOptions;
use board_plugin::resources::{BoardAssets, SpriteMaterial};
use board_plugin::BoardPlugin;

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    InGame,
    Paused,
    GameOver,
    Out,
}

#[derive(Deref, DerefMut)]
struct GameTime(Stopwatch);

impl GameTime {
    pub fn to_string(&self) -> String {
        let duration = self.elapsed();
        let minutes = (duration.as_secs() as f32 / 60.).floor() as u16;
        let seconds = (duration.as_secs() % 60) as u16;
        let minute_string = if minutes < 10 {
            format!("0{}", minutes)
        } else {
            format!("{}", minutes)
        };
        let second_string = if seconds < 10 {
            format!("0{}", seconds)
        } else {
            format!("{}", seconds)
        };

        format!("{}:{}", minute_string, second_string)
    }

    pub fn new_paused() -> Self {
        let mut game_time_watch = Stopwatch::new();
        game_time_watch.pause();
        Self(game_time_watch)
    }
}

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

fn game_time_system(
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

#[derive(Component)]
struct PauseScreen;

fn pause_screen(
    mut commands: Commands,
    windows: Res<Windows>,
    game_time: ResMut<GameTime>,
    board_assets: Res<BoardAssets>,
) {
    let window = windows.get_primary().unwrap();

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0., 0., 0., 0.8),
                custom_size: Some(vec2(window.width(), window.height())),
                ..Default::default()
            },
            transform: Transform::from_xyz(0., 0., 4.),
            ..Default::default()
        })
        .insert(PauseScreen)
        .insert(Name::new("PauseScreen"))
        .with_children(|parent| {
            parent.spawn_bundle(Text2dBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: format!("PAUSED - {}", game_time.to_string()),
                        style: TextStyle {
                            font: board_assets.bomb_counter_font.clone(),
                            font_size: 100.,
                            color: Color::WHITE,
                        },
                    }],
                    alignment: TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                },
                transform: Transform::from_xyz(0., 0., 1.),
                ..Default::default()
            });
        });
}

fn clear_pause_screen(mut commands: Commands, pause: Query<Entity, With<PauseScreen>>) {
    for pause_screen in pause.iter() {
        commands.entity(pause_screen).despawn_recursive();
    }
}

fn setup_board(
    mut commands: Commands,
    mut state: ResMut<State<AppState>>,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(BoardOptions {
        map_size: (20, 20),
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
