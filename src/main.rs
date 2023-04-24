mod animate;
mod camera;
mod chunks;
mod planet;
mod spaceship;

use bevy::{
	diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
	prelude::*,
	window::WindowMode,
};
use bevy_egui::EguiPlugin;
use camera::{camera_player_lock, setup_camera};
use chunks::render_chunks;
use planet::animate_planets;
use spaceship::{player_movement_system, setup_spaceship};

fn main() {
	let _app = App::new()
		.insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
		.add_plugins(
			DefaultPlugins
				.set(ImagePlugin::default_nearest())
				.set(WindowPlugin {
					primary_window: Some(Window {
						title: "Universe Generator".into(),
						mode: WindowMode::Windowed,
						..default()
					}),
					..default()
				}),
		)
		.add_plugin(FrameTimeDiagnosticsPlugin::default())
		.add_plugin(EguiPlugin)
		.add_startup_system(setup_camera)
		.add_startup_system(setup_spaceship)
		.add_startup_system(setup_fps_counter)
		.add_system(update_fps_counter)
		.add_system(render_chunks)
		.add_system(player_movement_system)
		.add_system(camera_player_lock)
		.add_system(animate_planets)
		.run();
}

#[derive(Component)]
struct FpsText;

fn setup_fps_counter(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands.spawn((
		TextBundle::from_sections([
			TextSection::new(
				"FPS: ",
				TextStyle {
					font: asset_server.load("Minecraft.ttf"),
					font_size: 60.0,
					color: Color::WHITE,
				},
			),
			TextSection::from_style(TextStyle {
				font: asset_server.load("Minecraft.ttf"),
				font_size: 60.0,
				color: Color::GOLD,
			}),
		]),
		FpsText,
	));
}

fn update_fps_counter(
	diagnostics: Res<Diagnostics>,
	mut text_query: Query<&mut Text, With<FpsText>>,
) {
	for mut text in &mut text_query {
		if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
			if let Some(value) = fps.smoothed() {
				text.sections[1].value = format!("{:.2}", value);
			}
		}
	}
}
