use bevy::prelude::{Component, Deref, DerefMut, TextureAtlasSprite, Time, Timer};

#[derive(Component)]
pub struct AnimationIndices {
	pub first: usize,
	pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

pub trait Animate {
	fn animate(
		sprite: &mut TextureAtlasSprite,
		animation_indices: &AnimationIndices,
		animation_timer: &mut AnimationTimer,
		time: &Time,
	);
}
