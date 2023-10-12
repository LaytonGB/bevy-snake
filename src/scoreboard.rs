use bevy::{ecs::query::WorldQuery, prelude::*};
use bevy_egui::{egui, EguiContexts};

#[derive(Component)]
pub struct ScoreboardEntry {
    name: String,
    score: u32,
}

pub fn setup_scoreboard(mut commands: Commands) {
    // TODO load from file into hashmap
    let scores: Vec<(String, u32)> = Vec::new();
    scores.iter().for_each(|(name, score)| {
        commands.spawn(ScoreboardEntry {
            name: name.clone(),
            score: *score,
        });
    });
}

pub fn update_scoreboard_display(
    mut commands: Commands,
    mut contexts: EguiContexts,
    scores: Query<&ScoreboardEntry>,
) {
    egui::Window::new("Scoreboard").show(contexts.ctx_mut(), |ui| {
        ui.with_layout(
            egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
            |ui| {
                for score in &scores {
                    ui.label(&score.name);
                    ui.label(format!("{}", score.score));
                    ui.end_row();
                }
            },
        )
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

    egui::Window::new("Scoreboard").show(context.ctx_mut(), |ui| {
        ui.centered_and_justified(|ui| {
            ui.text_edit_singleline(&mut user_input_string);
        });
    });
}
