use bevy::prelude::*;

#[derive(Debug, Resource)]
pub struct PlayerLives {
    pub play1: i8,
    pub play2: i8,
    pub play3: i8,
}
