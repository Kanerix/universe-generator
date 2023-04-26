use bevy::{prelude::*, window::PrimaryWindow};

use crate::spaceship::Spaceship;

const TILE_SIZE: f32 = 150.;
const EXTRA_TILES: f32 = 2.;

#[derive(Component)]
pub struct Tile {
	pub x: i32,
	pub y: i32,
}

pub struct TilePlugin;

impl Plugin for TilePlugin {
	fn build(&self, app: &mut App) {
		app.add_system(render_tiles).add_system(cleanup_tiles);
	}
}

pub fn render_tiles(
	mut commands: Commands,
	tiles_query: Query<&Tile, With<Tile>>,
	spaceship_query: Query<&Transform, With<Spaceship>>,
	primary_window_query: Query<&Window, With<PrimaryWindow>>,
	asset_server: Res<AssetServer>,
) {
	let Ok(primary_window) = primary_window_query.get_single() else {
		return;
	};
	let Ok(spaceship_transform) = spaceship_query.get_single() else {
		return;
	};

	let x_tile_offset: i32 = f32::round(spaceship_transform.translation.x / TILE_SIZE) as i32;
	let y_tile_offset: i32 = f32::round(spaceship_transform.translation.y / TILE_SIZE) as i32;
	let x_tiles_amount = f32::ceil(primary_window.width() / TILE_SIZE + EXTRA_TILES);
	let y_tiles_amount = f32::ceil(primary_window.height() / TILE_SIZE + EXTRA_TILES);

	for x in x_tile_offset..x_tiles_amount as i32 + x_tile_offset {
		for y in y_tile_offset..y_tiles_amount as i32 + y_tile_offset {
			if tiles_query.iter().any(|tile| tile.x == x && tile.y == y) {
				continue;
			}

			commands.spawn((
				SpriteBundle {
					texture: asset_server.load("chunk.png"),
					sprite: Sprite {
						custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
						..default()
					},
					transform: Transform::from_xyz(
						(x as f32 * TILE_SIZE) - (x_tiles_amount * TILE_SIZE / 2.)
							+ (TILE_SIZE / 2.),
						(y as f32 * TILE_SIZE) - (y_tiles_amount * TILE_SIZE / 2.)
							+ (TILE_SIZE / 2.),
						0.,
					),
					..default()
				},
				Tile { x, y },
			));
		}
	}
}

pub fn cleanup_tiles(
	mut commands: Commands,
	spaceship_query: Query<&Transform, With<Spaceship>>,
	tiles_query: Query<(Entity, &Tile), With<Tile>>,
	primary_window_query: Query<&Window, With<PrimaryWindow>>,
) {
	let Ok(primary_window) = primary_window_query.get_single() else {
		return;
	};
	let Ok(spaceship_transform) = spaceship_query.get_single() else {
		return;
	};

	let x_tile_offset: i32 = f32::round(spaceship_transform.translation.x / TILE_SIZE) as i32;
	let y_tile_offset: i32 = f32::round(spaceship_transform.translation.y / TILE_SIZE) as i32;
	let x_tiles_amount = f32::ceil(primary_window.width() / TILE_SIZE + EXTRA_TILES);
	let y_tiles_amount = f32::ceil(primary_window.height() / TILE_SIZE + EXTRA_TILES);

	for (tile_entity, tile) in tiles_query.iter() {
		if (tile.x < x_tile_offset - 1 || tile.x > x_tile_offset + x_tiles_amount as i32 + 1)
			|| (tile.y < y_tile_offset - 1 || tile.y > y_tile_offset + y_tiles_amount as i32 + 1)
		{
			commands.entity(tile_entity).despawn_recursive();
		}
	}
}
