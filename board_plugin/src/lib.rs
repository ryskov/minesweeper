pub mod components;
pub mod resources;

use bevy::log;
use bevy::prelude::*;
use resources::tilemap::TileMap;
use resources::BoardOptions;

use crate::resources::BoardPosition;
use crate::resources::TileSize;
use crate::components::Coordinates;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::create_board);
        log::info!("Loaded Board Plugin");
    }
}

impl BoardPlugin {
    pub fn create_board(
        mut commands: Commands,
        board_options: Option<Res<BoardOptions>>,
        window: Res<WindowDescriptor>,
    ) {
        let options = match board_options {
            None => BoardOptions::default(),
            Some(o) => o.clone(),
        };

        let mut tile_map = TileMap::empty(options.map_size.0, options.map_size.1);
        tile_map.set_bombs(options.bomb_count);

        #[cfg(feature = "debug")]
        log::info!("{}", tile_map.console_output());

        let tile_size = match options.tile_size {
            TileSize::Fixed(v) => v,
            TileSize::Adaptive { min, max } => Self::adaptive_tile_size(
                window,
                (min, max),
                (options.map_size.0, options.map_size.1),
            ),
        };

        let board_size = Vec2::new(
            options.map_size.0 as f32 * tile_size,
            options.map_size.1 as f32 * tile_size,
        );
        log::info!("board size: {}", board_size);

        let board_position = match options.position {
            BoardPosition::Centered { offset } => {
                Vec3::new(-(board_size.x / 2.), -(board_size.y / 2.), 0.) + offset
            }
            BoardPosition::Custom(p) => p,
        };

        commands
            .spawn()
            .insert(Name::new("Board"))
            .insert(Transform::from_translation(board_position))
            .insert(GlobalTransform::default())
            .with_children(|parent| {
                parent
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::WHITE,
                            custom_size: Some(board_size),
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(board_size.x / 2., board_size.y / 2., 0.),
                        ..Default::default()
                    })
                    .insert(Name::new("Background"));

                for (y, line) in tile_map.iter().enumerate() {
                    for (x, _tile) in line.iter().enumerate() {
                        parent
                            .spawn_bundle(SpriteBundle {
                                sprite: Sprite {
                                    color: Color::GRAY,
                                    custom_size: Some(Vec2::splat(
                                        tile_size - options.tile_padding as f32,
                                    )),
                                    ..Default::default()
                                },
                                transform: Transform::from_xyz(
                                    (x as f32 * tile_size) + (tile_size / 2.),
                                    (y as f32 * tile_size) + (tile_size / 2.),
                                    1.,
                                ),
                                ..Default::default()
                            })
                            .insert(Name::new(format!("Tile ({}, {})", x, y)))
                            .insert(Coordinates {
                                x: x as u16,
                                y: y as u16,
                            });
                    }
                }
            });
    }

    fn adaptive_tile_size(
        window: Res<WindowDescriptor>,
        (min, max): (f32, f32),
        (width, height): (u16, u16),
    ) -> f32 {
        let max_width = window.width / width as f32;
        let max_height = window.height / height as f32;
        max_width.min(max_height).clamp(min, max)
    }
}
