use bevy::{prelude::{Component, Commands, NextState}, utils::HashMap};
use bevy_egui::{egui, EguiContexts};

use crate::state::GameState;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Score(u32);

pub fn setup_scoreboard(mut state: ResMut<NextState<GameState>>) {
    // TODO load from file into hashmap
    let scores: HashMap<String, u32> = HashMap::new();
    if scores.values().iter().fold(Vec::with_capacity(10), |mut vec, score| {
        // TODO keep top 10
    })
}

pub fn scoreboard_entry(mut context: EguiContexts) {
    let mut user_input_string = String::new();

    egui::Window::new("Scoreboard").show(context.ctx_mut(), |ui| {
        ui.centered_and_justified(|ui| {
            ui.text_edit_singleline(&mut user_input_string);
        });
    });
}
