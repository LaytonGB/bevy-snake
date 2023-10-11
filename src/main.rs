mod buttons;
mod main_menu;
mod play_system;
mod state;
mod window;

use bevy::{prelude::*, window::PresentMode, winit::WinitSettings};
use bevy_egui::EguiPlugin;
use buttons::*;
use main_menu::*;
use play_system::*;
use state::*;
use window::*;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        // config
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Snake game".into(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                present_mode: PresentMode::AutoNoVsync,
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                // TODO when bevy version upgrades, enable these
                // enabled_buttons: bevy::window::EnabledButtons {
                //     maximize: false,
                //     ..Default::default()
                // },
                // visible: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin)
        .insert_resource(WinitSettings::game()) // react constantly - needed for egui
        // setup
        .add_state::<GameState>()
        .add_plugins(ButtonSystem)
        .add_systems(Startup, setup_camera)
        // menu
        .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
        .add_systems(OnExit(GameState::MainMenu), teardown_main_menu)
        // play
        .add_systems(OnEnter(GameState::Playing), setup_play)
        .add_systems(Update, draw_grid.run_if(in_state(GameState::Playing)))
        .add_systems(OnExit(GameState::Playing), teardown_play)
        .run();
}
