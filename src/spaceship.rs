use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

const MAX_ALLOWED_SPEED: f32 = 400.;
const MIN_ALLOWED_SPEED: f32 = 0.;

#[derive(Component)]
pub struct Spaceship {
	speed: f32,
	acceleration: f32,
	resistance: f32,
	turning_force: f32,
}

impl Default for Spaceship {
	fn default() -> Self {
		Self {
			speed: MIN_ALLOWED_SPEED,
			acceleration: 300.,
			resistance: 150.,
			turning_force: 0.01,
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

pub fn player_movement(
	mut player_query: Query<(&mut Transform, &mut Spaceship), With<Spaceship>>,
	input: Res<Input<KeyCode>>,
	time: Res<Time>,
) {
	let Ok((mut transform, mut spaceship)) = player_query.get_single_mut() else {
		return;
	};

	let delta_seconds = time.delta_seconds();
	if spaceship.speed <= MAX_ALLOWED_SPEED && spaceship.speed >= MIN_ALLOWED_SPEED {
		let speed = (spaceship.speed + spaceship.acceleration) * delta_seconds;
		if input.pressed(KeyCode::W) {
			spaceship.speed += speed;
		}
		if input.pressed(KeyCode::S) {
			spaceship.speed -= speed;
		}

		spaceship.speed -= spaceship.resistance * delta_seconds;

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

pub fn player_debug(
	mut contexts: EguiContexts,
	mut player_query: Query<(&mut Transform, &mut Spaceship), With<Spaceship>>,
) {
	let Ok(player) = player_query.get_single_mut() else {
		return;
	};

	egui::Window::new("Player").show(contexts.ctx_mut(), |ui| {
		ui.label("Position");
		ui.label(format!("X: {}", player.0.translation.x));
		ui.label(format!("Y: {}", player.0.translation.y));
	});
}
