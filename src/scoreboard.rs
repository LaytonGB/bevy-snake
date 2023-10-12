use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use egui::Window;
use serde::*;

#[derive(Component, Deserialize)]
pub struct ScoreboardEntry {
    name: String,
    score: u32,
}

pub fn setup_scoreboard(mut commands: Commands) {
    let csv_text = std::fs::read_to_string("assets/data/scoreboard.csv")
        .expect("failed to read scoreboard file");
    let mut reader = csv::Reader::from_reader(csv_text.as_bytes());
    let mut scores = reader.deserialize::<ScoreboardEntry>();
    while let Some(Ok(ScoreboardEntry { name, score })) = scores.next() {
        commands.spawn(ScoreboardEntry {
            name: name,
            score: score,
        });
    }
}

pub fn update_scoreboard_display(mut contexts: EguiContexts, scores: Query<&ScoreboardEntry>) {
    Window::new("Scoreboard")
        .resizable(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.horizontal_wrapped(|ui| {
                for score in &scores {
                    ui.label(&score.name);
                    ui.label(format!("{}", score.score));
                    ui.end_row();
                }
            });
        });
}

pub fn teardown_scoreboard(
    mut commands: Commands,
    scoreboard: Query<Entity, With<ScoreboardEntry>>,
) {
    for score in &scoreboard {
        commands.entity(score).despawn();
    }
}

pub fn scoreboard_entry(mut context: EguiContexts) {
    let mut user_input_string = String::new();

    Window::new("Scoreboard").show(context.ctx_mut(), |ui| {
        ui.centered_and_justified(|ui| {
            ui.text_edit_singleline(&mut user_input_string);
        });
    });
}
