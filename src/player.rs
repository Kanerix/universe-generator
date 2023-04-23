use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub fn setup_player(
	mut commands: Commands,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	asset_server: ResMut<AssetServer>,
) {
	let img_handle = asset_server.load("spaceship.png");
	let texture_atlas = TextureAtlas::from_grid(img_handle, Vec2::splat(48.), 1, 1, None, None);
	let texture_atlas_handle = texture_atlases.add(texture_atlas);

	commands
		.spawn(SpriteSheetBundle {
			texture_atlas: texture_atlas_handle,
			sprite: TextureAtlasSprite {
				custom_size: Some(Vec2::splat(256.)),
				..default()
			},
			..default()
		})
		.insert(Player);
}

pub fn player_movement_system(
	mut player_query: Query<&mut Transform, With<Player>>,
	input: Res<Input<KeyCode>>,
	time: Res<Time>,
) {
	let mut player_transform = player_query.single_mut();
	let mut new_pos = Vec3::ZERO;

	if input.pressed(KeyCode::W) {
		new_pos.y += 1.;
	}
	if input.pressed(KeyCode::S) {
		new_pos.y -= 1.;
	}
	if input.pressed(KeyCode::D) {
		new_pos.x += 1.;
	}
	if input.pressed(KeyCode::A) {
		new_pos.x -= 1.;
	}

	player_transform.translation += new_pos.normalize_or_zero() * time.delta_seconds() * 200.;
}
