mod buttons;
mod main_menu;
mod state;

use bevy::{prelude::*, winit::WinitSettings};
use buttons::*;
use main_menu::*;
use state::*;

fn main() {
    App::new()
        .insert_resource(WinitSettings::game()) // react constantly
        // .insert_resource(WinitSettings::desktop_app()) // react to interactions
        .add_plugins(DefaultPlugins)
        .add_state::<GameState>()
        // .add_systems(Startup, setup_main_menu)
        .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
        .add_systems(OnExit(GameState::MainMenu), teardown_main_menu)
        // .add_systems(OnEnter(GameState::Playing), systems)
        // .add_systems(Update, button_system)
        .add_plugins(ButtonSystem)
        .run();
}
