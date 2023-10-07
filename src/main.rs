mod buttons;
mod main_menu;
mod play_system;
mod state;

use bevy::{prelude::*, winit::WinitSettings};
use buttons::*;
use main_menu::*;
use play_system::*;
use state::*;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings::game()) // react constantly
        // .insert_resource(WinitSettings::desktop_app()) // react to interactions
        .add_state::<GameState>()
        .add_plugins(ButtonSystem)
        .add_systems(Startup, setup_camera)
        .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
        .add_systems(OnExit(GameState::MainMenu), teardown_main_menu)
        .add_systems(OnEnter(GameState::Playing), setup_play)
        .add_systems(OnExit(GameState::Playing), teardown_play)
        .run();
}
