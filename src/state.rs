use bevy::prelude::*;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, States)]
pub enum GameState {
    #[default]
    MainMenu,
    Playing,
    Paused,
    GameOver,
}
