use bevy::prelude::*;

use crate::{
	animate::{Animate, AnimationIndices, AnimationTimer},
	chunks::Chunk,
};

#[derive(Component)]
pub struct Planet;

impl Animate for Planet {
	fn animate(
		mut sprite: &mut TextureAtlasSprite,
		indices: &AnimationIndices,
		timer: &mut AnimationTimer,
		time: &Time,
	) {
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

pub fn render_planets(
	mut commands: Commands,
	chunks_query: Query<Entity, With<Chunk>>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	asset_server: ResMut<AssetServer>,
) {
	for entity in chunks_query.iter() {
		let img_handle = asset_server.load("planets/earth.png");
		let texture_atlas =
			TextureAtlas::from_grid(img_handle, Vec2::splat(100.), 50, 1, None, None);
		let texture_atlas_handle = texture_atlases.add(texture_atlas);

		commands
			.entity(entity)
			.with_children(|builder| {
				builder.spawn((
					SpriteSheetBundle {
						texture_atlas: texture_atlas_handle,
						sprite: TextureAtlasSprite::new(0),
						transform: Transform::from_xyz(0., 0., 0.5),
						..default()
					},
					AnimationIndices { first: 0, last: 49 },
					AnimationTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
				));
			})
			.insert(Planet);
	}
}

pub fn animate_planets(
	planets_query: Query<&Children, With<Planet>>,
	mut planets_children_query: Query<(
		&mut TextureAtlasSprite,
		&AnimationIndices,
		&mut AnimationTimer,
	)>,
	time: Res<Time>,
) {
	for children in planets_query.iter() {
		for entity in children.iter() {
			if let Ok(child_component) = planets_children_query.get_mut(*entity) {
				let (mut sprite, indices, mut timer) = child_component;
				Planet::animate(&mut sprite, &indices, &mut timer, &time);
			}
		}
	}
}
