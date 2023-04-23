mod earth;

use bevy::prelude::*;

use crate::animate::{AnimationIndices, AnimationTimer};

pub trait PlanetAnimations {
	fn spin();
}

#[derive(Component)]
pub struct Planet {
	name: String,
}

pub fn spawn_planet(
	mut commands: Commands,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	asset_server: ResMut<AssetServer>,
) {
	let img_handle = asset_server.load("planets/earth.png");
	let texture_atlas = TextureAtlas::from_grid(img_handle, Vec2::splat(100.), 50, 1, None, None);
	let texture_atlas_handle = texture_atlases.add(texture_atlas);

	commands
		.spawn((
			SpriteSheetBundle {
				texture_atlas: texture_atlas_handle,
				sprite: TextureAtlasSprite::new(0),
				transform: Transform::from_scale(Vec3::splat(5.)),
				..default()
			},
			AnimationIndices { first: 0, last: 49 },
			AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
		))
		.insert(Planet {
			name: "Earth".to_string(),
		});
}

pub fn animate_planets(
	mut planets_query: Query<
		(
			&AnimationIndices,
			&mut AnimationTimer,
			&mut TextureAtlasSprite,
		),
		With<Planet>,
	>,
	time: Res<Time>,
) {
	for (indices, mut timer, mut sprite) in planets_query.iter_mut() {
		timer.tick(time.delta());
		if timer.just_finished() {
			sprite.index = if sprite.index == indices.last {
				indices.first
			} else {
				sprite.index + 1
			}
		}
	}
}
