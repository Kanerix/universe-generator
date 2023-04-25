mod animate;
mod camera;
mod chunks;
mod planet;
mod spaceship;

use bevy::{prelude::*, window::WindowMode};
use bevy_egui::EguiPlugin;
use camera::{camera_player_lock, setup_camera};
use chunks::render_chunks;
use planet::{animate_planets, render_planets};
use spaceship::{player_debug, player_movement, setup_spaceship};

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
		.add_plugin(EguiPlugin)
		.add_startup_system(setup_camera)
		.add_startup_system(setup_spaceship)
		.add_system(render_chunks)
		.add_system(render_planets)
		.add_system(animate_planets)
		.add_system(player_movement)
		.add_system(player_debug)
		.add_system(camera_player_lock)
		.run();
}
