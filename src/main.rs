mod animate;
mod camera;
mod planet;
mod player;
mod universe;

use bevy::{prelude::*, window::WindowMode};
use bevy_egui::EguiPlugin;
use camera::{camera_player_lock, setup_camera};
use planet::{spawn_planet, animate_planets};
use player::{player_movement_system, setup_player};

fn main() {
	let _app = App::new()
		.insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
		.add_plugins(
			DefaultPlugins
				.set(ImagePlugin::default_nearest())
				.set(WindowPlugin {
					primary_window: Some(Window {
						title: "Universe Generator".into(),
						mode: WindowMode::Fullscreen,
						resizable: false,
						..default()
					}),
					..default()
				}),
		)
		.add_plugin(EguiPlugin)
		.add_startup_system(setup_camera)
		.add_startup_system(setup_player)
		.add_startup_system(spawn_planet)
		.add_system(player_movement_system)
		.add_system(camera_player_lock)
		.add_system(animate_planets)
		.run();
}
