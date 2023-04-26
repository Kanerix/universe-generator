use bevy::{prelude::*, window::PrimaryWindow};

use crate::spaceship::Spaceship;

const CHUNK_SIZE: f32 = 150.;

#[derive(Component)]
pub struct Chunk {
	x: i32,
	y: i32,
}

pub fn render_chunks(
	mut commands: Commands,
	chunks_query: Query<(Entity, &Chunk), With<Chunk>>,
	spaceship_query: Query<&Transform, With<Spaceship>>,
	primary_window_query: Query<&Window, With<PrimaryWindow>>,
	asset_server: Res<AssetServer>,
) {
	let Ok(primary_window) = primary_window_query.get_single() else {
		return;
	};
	let Ok(spaceship) = spaceship_query.get_single() else {
		return;
	};

	let chunk_offset_x: i32 = f32::round(spaceship.translation.x / CHUNK_SIZE) as i32;
	let chunk_offset_y: i32 = f32::round(spaceship.translation.y / CHUNK_SIZE) as i32;

	let extra_chunks = -2.;
	let horizontal_chunks_amount = f32::ceil(primary_window.width() / CHUNK_SIZE + extra_chunks);
	let vertical_chunks_amount = f32::ceil(primary_window.height() / CHUNK_SIZE + extra_chunks);

	for x in chunk_offset_x..horizontal_chunks_amount as i32 + chunk_offset_x {
		for y in chunk_offset_y..vertical_chunks_amount as i32 + chunk_offset_y {
			if chunks_query.iter().any(|(_, chunk)| chunk.x == x && chunk.y == y) {
				continue
			}

			commands.spawn((
				SpriteBundle {
					texture: asset_server.load("chunk.png"),
					sprite: Sprite {
						custom_size: Some(Vec2::new(CHUNK_SIZE, CHUNK_SIZE)),
						..default()
					},
					transform: Transform::from_xyz(
						(x as f32 * CHUNK_SIZE) - (horizontal_chunks_amount * CHUNK_SIZE / 2.)
							+ (CHUNK_SIZE / 2.),
						(y as f32 * CHUNK_SIZE) - (vertical_chunks_amount * CHUNK_SIZE / 2.)
							+ (CHUNK_SIZE / 2.),
						0.,
					),
					..default()
				},
				Chunk { x, y },
			));
		}
	}
}

pub fn culling_system(
    player_transform: Query<&Transform, With<Spaceship>>,
    chunk_transforms: Query<(Entity, &Transform), With<Chunk>>,
    mut commands: Commands,
) {
    let player_pos = player_transform.single().translation;

    for (chunk_entity, chunk_transform) in chunk_transforms.iter() {
        let chunk_pos = chunk_transform.translation;

        // Calculate the distance between the player and the chunk
        let distance = player_pos.distance(chunk_pos);

        // If the chunk is outside the render distance, remove it
        if distance > 600. {
            commands.entity(chunk_entity).despawn();
        }
    }
}