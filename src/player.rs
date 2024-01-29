use bevy::prelude::*;
use bevy_matchbox::prelude::*;
use serde::{Deserialize, Serialize};

use crate::card::Card;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PlayerState {
    LobbyFree,
    RoomWaiting,
    Gaming,
}

impl Default for PlayerState {
    fn default() -> Self {
        PlayerState::LobbyFree
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Resource, Component)]
pub struct Player {
    pub id: PeerId,
    pub state: PlayerState,
    pub room_index: Option<i32>,
    pub current_round: bool,
    pub hand: Vec<Card>,
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Player {}

impl Player {
    pub fn new(peer: PeerId) -> Self {
        Self {
            id: peer,
            room_index: None,
            current_round: false,
            state: PlayerState::default(),
            hand: Vec::with_capacity(51 / 3),
        }
    }
}
