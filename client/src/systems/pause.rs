use bevy::{prelude::*, math::vec2};
use board_plugin::resources::BoardAssets;

use crate::{resources::GameTime, components::PauseScreen};

pub fn pause_screen(
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

pub fn clear_pause_screen(mut commands: Commands, pause: Query<Entity, With<PauseScreen>>) {
    for pause_screen in pause.iter() {
        commands.entity(pause_screen).despawn_recursive();
    }
}