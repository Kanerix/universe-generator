use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow};

use crate::spaceship::Spaceship;

const CHUNK_SIZE: f32 = 50.;

#[derive(Component)]
pub struct Chunck {
	x: i32,
	y: i32,
}

pub fn render_chunks(
	mut commands: Commands,
	chunks_query: Query<(Option<Entity>, &Chunck), With<Chunck>>,
	spaceship_transform_query: Query<&Transform, With<Spaceship>>,
	primary_window_query: Query<&Window, With<PrimaryWindow>>,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<ColorMaterial>>,
) {
	let Ok(primary_window) = primary_window_query.get_single() else {
		return;
	};
	let Ok(spaceship_transform) = spaceship_transform_query.get_single() else {
		return;
	};

	let chunk_offset_x: i32 = f32::ceil(spaceship_transform.translation.x / CHUNK_SIZE) as i32;
	let chunk_offset_y: i32 = f32::ceil(spaceship_transform.translation.y / CHUNK_SIZE) as i32;

	let extra_chunks = -10.;
	let horizontal_chunks_amount = f32::ceil(primary_window.width() / CHUNK_SIZE + extra_chunks);
	let vertical_chunks_amount = f32::ceil(primary_window.height() / CHUNK_SIZE + extra_chunks);

	for x in chunk_offset_x..horizontal_chunks_amount as i32 + chunk_offset_x {
		for y in chunk_offset_y..vertical_chunks_amount as i32 + chunk_offset_x {
			let mut rendered = false;
			for (entity, chunk) in chunks_query.iter() {
				if chunk.x > chunk_offset_x + horizontal_chunks_amount as i32
					|| chunk.x < chunk_offset_x - horizontal_chunks_amount as i32
					|| chunk.y > chunk_offset_y + vertical_chunks_amount as i32
					|| chunk.y < chunk_offset_y - vertical_chunks_amount as i32
				{
					let Some(entity) = entity else {
						continue;
					};

					commands.entity(entity).despawn();
					continue;
				}

				if chunk.x == x && chunk.y == y {
					rendered = true;
					continue;
				}
			}

			if rendered {
				continue;
			}

			commands.spawn((
				MaterialMesh2dBundle {
					mesh: meshes
						.add(shape::Box::new(CHUNK_SIZE - 2., CHUNK_SIZE - 2., 0.).into())
						.into(),
					material: materials.add(ColorMaterial::from(Color::PURPLE)),
					transform: Transform::from_xyz(
						(x as f32 * CHUNK_SIZE) - (horizontal_chunks_amount * CHUNK_SIZE / 2.)
							+ (CHUNK_SIZE / 2.),
						(y as f32 * CHUNK_SIZE) - (vertical_chunks_amount * CHUNK_SIZE / 2.)
							+ (CHUNK_SIZE / 2.),
						0.,
					),
					..default()
				},
				Chunck { x, y },
			));
		}
	}
}
