use bevy::prelude::*;

#[allow(dead_code)]
#[derive(
    States, Default, Debug, Clone, Eq, PartialEq, Hash,
)]
pub enum GameModeState {
    Paused,
    #[default]
    Running,
}

#[derive(SystemSet, Debug, Clone, Eq, PartialEq, Hash)]
pub struct GameplaySet;
