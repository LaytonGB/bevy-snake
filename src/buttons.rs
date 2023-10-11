use bevy::{app::AppExit, prelude::*};

use crate::state::*;

pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component)]
pub enum ButtonAction {
    MENU,
    PLAY,
    SCOREBOARD,
    EXIT,
}

pub struct ButtonSystem;

impl Plugin for ButtonSystem {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, button_styles)
            .add_systems(Update, button_interactions);
    }
}

fn button_styles(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
    // mut text_query: Query<&mut Text>,
    // mut state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut background_color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

fn button_interactions(
    interaction_query: Query<(&Interaction, &ButtonAction), (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<NextState<GameState>>,
    mut app_exit_events: ResMut<Events<AppExit>>,
) {
    for (interaction, button_action) in &interaction_query {
        match button_action {
            ButtonAction::MENU => {
                if let Interaction::Pressed = interaction {
                    state.set(GameState::MainMenu);
                }
            }
            ButtonAction::PLAY => {
                if let Interaction::Pressed = interaction {
                    state.set(GameState::Playing);
                }
            }
            ButtonAction::SCOREBOARD => {
                if let Interaction::Pressed = interaction {
                    state.set(GameState::Scoreboard);
                }
            }
            ButtonAction::EXIT => {
                if let Interaction::Pressed = interaction {
                    app_exit_events.send(AppExit);
                }
            }
        }
    }
}
