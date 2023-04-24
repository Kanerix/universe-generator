use bevy::prelude::*;

use crate::spaceship::Spaceship;

#[derive(Component)]
pub struct Camera;

pub fn setup_camera(mut commands: Commands) {
	commands.spawn(Camera2dBundle::default()).insert(Camera);
}

pub fn camera_player_lock(
	mut camera_query: Query<&mut Transform, (With<Camera>, Without<Spaceship>)>,
	player_query: Query<&mut Transform, (With<Spaceship>, Without<Camera>)>,
	time: Res<Time>,
) {
	let player_transform = player_query.single();
	let mut camera_transform = camera_query.single_mut();

	let factor = 1.0 - f32::exp(-time.delta_seconds() * 5.0);
	let current_pos = camera_transform.translation;
	let target_pos = player_transform.translation;
	let new_pos = current_pos + (target_pos - current_pos) * factor;
	camera_transform.translation = new_pos;
}
