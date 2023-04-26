use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

const MAX_ALLOWED_SPEED: f32 = 300.;
const MIN_ALLOWED_SPEED: f32 = 66.;

#[derive(Component)]
pub struct Spaceship {
	speed: f32,
	acceleration: f32,
	resistance: f32,
	turning_force: f32,
	brake_force: f32,
}

impl Default for Spaceship {
	fn default() -> Self {
		Self {
			speed: MIN_ALLOWED_SPEED,
			acceleration: 233.,
			resistance: 150.,
			turning_force: 0.01,
			brake_force: 200.
		}
	}
}

pub fn setup_spaceship(mut commands: Commands, asset_server: ResMut<AssetServer>) {
	commands.spawn((
		SpriteBundle {
			texture: asset_server.load("spaceship.png"),
			transform: Transform::from_xyz(0., 0., 1.),
			..default()
		},
		Spaceship { ..default() },
	));
}

pub fn spaceship_movement(
	mut spaceship_query: Query<(&mut Transform, &mut Spaceship), With<Spaceship>>,
	input: Res<Input<KeyCode>>,
	time: Res<Time>,
) {
	let Ok((mut transform, mut spaceship)) = spaceship_query.get_single_mut() else {
		return;
	};

	let delta_seconds = time.delta_seconds();
	if spaceship.speed <= MAX_ALLOWED_SPEED && spaceship.speed >= MIN_ALLOWED_SPEED {
		if input.pressed(KeyCode::W) {
			spaceship.speed += (spaceship.speed + spaceship.acceleration) * delta_seconds;
		}
		if input.pressed(KeyCode::S) {
			spaceship.speed -= spaceship.brake_force * delta_seconds;
		}

		spaceship.speed -= spaceship.resistance * delta_seconds;

		// Make sure the spaceship is within the speed limits 
		if spaceship.speed > MAX_ALLOWED_SPEED {
			spaceship.speed = MAX_ALLOWED_SPEED;
		} else if spaceship.speed < MIN_ALLOWED_SPEED {
			spaceship.speed = MIN_ALLOWED_SPEED;
		}
	}

	if spaceship.speed > MIN_ALLOWED_SPEED {
		if input.pressed(KeyCode::A) {
			transform.rotate(Quat::from_rotation_z(
				spaceship.turning_force * spaceship.speed * delta_seconds,
			));
		}
		if input.pressed(KeyCode::D) {
			transform.rotate(Quat::from_rotation_z(
				-spaceship.turning_force * spaceship.speed * delta_seconds,
			));
		}
	}

	let velocity = transform.up();
	transform.translation += velocity * spaceship.speed * delta_seconds;
}

pub fn spaceship_debug(
	mut contexts: EguiContexts,
	mut spaceship_query: Query<(&mut Transform, &mut Spaceship), With<Spaceship>>,
) {
	let Ok(player) = spaceship_query.get_single_mut() else {
		return;
	};

	egui::Window::new("Spaceship").show(contexts.ctx_mut(), |ui| {
		ui.label("Position");
		ui.label(format!("X: {}", player.0.translation.x));
		ui.label(format!("Y: {}", player.0.translation.y));
	});
}
