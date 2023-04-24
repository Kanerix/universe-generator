use crate::animate::{Animate, AnimationIndices, AnimationTimer};
use bevy::{prelude::*, window::PrimaryWindow};

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
				transform: Transform::from_xyz(200., 200., 0.),
				..default()
			},
			AnimationIndices { first: 0, last: 49 },
			AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
		))
		.insert(Planet);
}

pub fn animate_planets(
	mut planets_query: Query<
		(
			&mut TextureAtlasSprite,
			&AnimationIndices,
			&mut AnimationTimer,
		),
		With<Planet>,
	>,
	time: Res<Time>,
) {
	for (mut sprite, indices, mut timer) in planets_query.iter_mut() {
		Planet::animate(&mut sprite, &indices, &mut timer, &time)
	}
}
