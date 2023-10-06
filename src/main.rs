mod buttons;
mod main_menu;
mod state;

use bevy::{prelude::*, winit::WinitSettings};
use buttons::*;
use main_menu::*;
use state::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings::desktop_app())
        .add_state::<GameState>()
        // .add_systems(Startup, setup_main_menu)
        .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
        .add_systems(OnExit(GameState::MainMenu), teardown_main_menu)
        // .add_systems(OnEnter(GameState::Playing), systems)
        .add_systems(Update, button_system)
        .run();
}
