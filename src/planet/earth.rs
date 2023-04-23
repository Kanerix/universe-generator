use bevy::prelude::Component;

use super::PlanetAnimations;

#[derive(Component)]
pub struct Earth {
	pub inhabitants: u32,
}

impl PlanetAnimations for Earth {
	fn spin() {
		todo!()
	}
}
